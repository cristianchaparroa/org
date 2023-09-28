import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import OrganizationFactory from "./typedContract/constructors/organization";
import Organization from "./typedContract/contracts/organization";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("organization test", () => {
  let organizationFactory: OrganizationFactory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  
  let contract: Organization;
  const initialState = true;

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");

    organizationFactory = new OrganizationFactory(api, deployer);

    contract = new Organization(
      (await organizationFactory.new(initialState)).address,
      deployer,
      api
    );
  });

  after(async function tearDown() {
    await api.disconnect();
  });
});
