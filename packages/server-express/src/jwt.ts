import { jwtVerify, importSPKI, JWTVerifyOptions, JWTPayload } from "jose";

export default async function validate<T extends JWTPayload>(
  token: string,
  { issuer, audience }: JWTVerifyOptions,
): Promise<T> {
  const response = await fetch(`${issuer}/.well-known/openid-configuration`, {
    headers: { Accept: "text/plain" },
  });
  const spki_pem = await response.text();
  const key = await importSPKI(spki_pem, "RS256");
  const { payload } = await jwtVerify(token, key, {
    algorithms: ["RS256"],
    issuer,
    audience,
  });

  return payload as T;
}
