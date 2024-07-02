import canonicalize from "canonicalize";
import pickBy from "lodash.pickby";

export interface DerivationPath {
  chain: number;
  domain?: string;
  meta?: Record<string, unknown>;
}

export const getCanonicalizedDerivationPath = (
  derivationPath: DerivationPath
) =>
  canonicalize(
    pickBy(
      {
        chain: derivationPath.chain,
        domain: derivationPath.domain,
        meta: derivationPath.meta,
      },
      (v: unknown): boolean => v !== undefined && v !== null
    )
  ) ?? "";
