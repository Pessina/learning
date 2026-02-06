"use strict";
/* eslint-disable-next-line no-unused-vars */
function __export(m) {
/* eslint-disable-next-line no-prototype-builtins */
    for (var p in m) if (!exports.hasOwnProperty(p)) exports[p] = m[p];
}
Object.defineProperty(exports, "__esModule", { value: true });
/* eslint-disable-next-line no-unused-vars */
var jtv = require('@mojotech/json-type-validation');
/* eslint-disable-next-line no-unused-vars */
var damlTypes = require('@daml/types');
/* eslint-disable-next-line no-unused-vars */
var damlLedger = require('@daml/ledger');

var pkgd27a1030f23715794aedb59a36c507a35d32fa56df68065f343b6c02acf9cd68 = require('@daml.js/daml-script-2.10.3');


exports.TestUser = {
  decoder: damlTypes.lazyMemo(function () { return jtv.object({alias: damlTypes.Text.decoder, public: damlTypes.Party.decoder, participantName: damlTypes.Optional(pkgd27a1030f23715794aedb59a36c507a35d32fa56df68065f343b6c02acf9cd68.Daml.Script.ParticipantName).decoder, }); }),
  encode: function (__typed__) {
  return {
    alias: damlTypes.Text.encode(__typed__.alias),
    public: damlTypes.Party.encode(__typed__.public),
    participantName: damlTypes.Optional(pkgd27a1030f23715794aedb59a36c507a35d32fa56df68065f343b6c02acf9cd68.Daml.Script.ParticipantName).encode(__typed__.participantName),
  };
}
,
};



exports.Parties = {
  decoder: damlTypes.lazyMemo(function () { return jtv.object({alice: damlTypes.Party.decoder, bob: damlTypes.Party.decoder, charlie: damlTypes.Party.decoder, public: damlTypes.Party.decoder, }); }),
  encode: function (__typed__) {
  return {
    alice: damlTypes.Party.encode(__typed__.alice),
    bob: damlTypes.Party.encode(__typed__.bob),
    charlie: damlTypes.Party.encode(__typed__.charlie),
    public: damlTypes.Party.encode(__typed__.public),
  };
}
,
};

