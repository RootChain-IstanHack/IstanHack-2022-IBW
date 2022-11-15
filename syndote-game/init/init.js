import { GearApi, GearKeyring } from '@gear-js/api';
import { readFileSync, existsSync } from 'fs';

const pathToOpt = process.argv[2] || '../program/target/wasm32-unknown-unknown/release/syndote.opt.wasm';
const providerAddress = process.argv.length > 3 ? process.argv[3] : process.argv[2];

function waitForInitialization(api, programId) {
  return new Promise((resolve) => {
    let messageId;
    api.query.system.events((events) => {
      for (const { event } of events) {
        if (event.method === 'MessageEnqueued') {
          if (event.data.destination.toHex() === programId && event.data.entry.toString() === 'Init') {
            messageId = event.data.id.toHex();
          }
          continue;
        }
        if (event.method === 'MessagesDispatched') {
          for (const [id, status] of event.data.statuses) {
            if (id.toHex() === messageId) {
              if (status.isSuccess) {
                resolve(true);
              }
              if (status.isFailed) {
                resolve(false);
              }
            }
          }
        }
      }
    });
  });
}

async function uploadSyndote(api) {
  if (!existsSync(pathToOpt)) {
    throw new Error("File doesn't exist");
  }
  const code = readFileSync(pathToOpt);
  const alice = await GearKeyring.fromSuri('//Alice');

  const { programId } = api.program.upload({ code, gasLimit: 5_000_000_000 });

  const isInitSuccess = waitForInitialization(api, programId);

  await new Promise((resolve, reject) =>
    api.program.signAndSend(alice, ({ events, status }) =>
      events.forEach(({ event }) => {
        if (status.isFinalized && event.method === 'MessageEnqueued') {
          resolve(programId);
        }
        if (event.method === 'ExtrinsicFailed') {
          reject(api.getExtrinsicFailedError(event));
        }
      }),
    ),
  );

  if (await isInitSuccess) {
    return programId;
  } else {
    throw new Error('Program initializstion failed');
  }
}

const main = async () => {
  const api = await GearApi.create({ providerAddress });

  const programId = await uploadSyndote(api);

  console.log(`Program ID: ${programId}`);
};

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });
