import { jwtVerify, importSPKI } from "jose";

export default async function validate(token) {
  const response = await fetch(
    `${import.meta.env.VITE_API_URL}/.well-known/openid-configuration`,
    {
      headers: { Accept: "text/plain" },
    },
  );
  const spki_pem = await response.text();
  const key = await importSPKI(spki_pem, "RS256");
  const { payload } = await jwtVerify(token, key, {
    algorithms: ["RS256"],
    issuer: import.meta.env.VITE_API_URL,
    audience: import.meta.env.VITE_GAME_NAME,
  });

  return { userId: payload.sub };
}
