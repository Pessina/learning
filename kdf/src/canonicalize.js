import canonicalize from "canonicalize";

import pickBy from "lodash.pickby";

export const getCanonicalizedDerivationPath = (derivationPath) =>
  canonicalize(
    pickBy(
      {
        chain: derivationPath.chain,
        domain: derivationPath.domain,
        meta: derivationPath.meta,
      },
      (v) => v !== undefined && v !== null
    )
  ) ?? "";
