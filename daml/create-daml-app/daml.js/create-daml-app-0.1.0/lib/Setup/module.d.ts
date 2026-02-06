// Generated from Setup.daml
/* eslint-disable @typescript-eslint/camelcase */
/* eslint-disable @typescript-eslint/no-namespace */
/* eslint-disable @typescript-eslint/no-use-before-define */
import * as jtv from '@mojotech/json-type-validation';
import * as damlTypes from '@daml/types';
/* eslint-disable-next-line @typescript-eslint/no-unused-vars */
import * as damlLedger from '@daml/ledger';

import * as pkgd27a1030f23715794aedb59a36c507a35d32fa56df68065f343b6c02acf9cd68 from '@daml.js/daml-script-2.10.3';

export declare type TestUser = {
  alias: string;
  public: damlTypes.Party;
  participantName: damlTypes.Optional<pkgd27a1030f23715794aedb59a36c507a35d32fa56df68065f343b6c02acf9cd68.Daml.Script.ParticipantName>;
};

export declare const TestUser:
  damlTypes.Serializable<TestUser> & {
  }
;


export declare type Parties = {
  alice: damlTypes.Party;
  bob: damlTypes.Party;
  charlie: damlTypes.Party;
  public: damlTypes.Party;
};

export declare const Parties:
  damlTypes.Serializable<Parties> & {
  }
;

