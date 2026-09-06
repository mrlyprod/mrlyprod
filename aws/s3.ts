import { S3Client } from "bun";

/* WHERE */

export const REGION = process.env.AWS_REGION || process.env.AWS_DEFAULT_REGION || "us-east-2";
export const NET_BUCKET = process.env.MRLYNET_BUCKET || "mrlynet";
export const DEV_BUCKET = process.env.MRLYDEV_BUCKET || "mrlydev";
export const PROD_BUCKET = process.env.MRLYPROD_BUCKET || "mrlyprod";

/* CREDENTIALS */

export type Creds = {
  accessKeyId: string;
  secretAccessKey: string;
  sessionToken?: string;
  region: string;
};

let held: Creds | null = null;

export function credentials(): Creds {
  if (held) return held;
  const id = process.env.AWS_ACCESS_KEY_ID;
  const secret = process.env.AWS_SECRET_ACCESS_KEY;
  if (id && secret) {
    held = { accessKeyId: id, secretAccessKey: secret, sessionToken: process.env.AWS_SESSION_TOKEN, region: REGION };
    return held;
  }
  const run = Bun.spawnSync(["aws", "configure", "export-credentials", "--format", "process"]);
  if (run.exitCode !== 0) throw new Error("s3: no AWS_* env credentials and aws configure export-credentials failed");
  const data = JSON.parse(run.stdout.toString()) as { AccessKeyId: string; SecretAccessKey: string; SessionToken?: string };
  held = {
    accessKeyId: data.AccessKeyId,
    secretAccessKey: data.SecretAccessKey,
    sessionToken: data.SessionToken,
    region: REGION,
  };
  return held;
}

/* CLIENT */

export function client(bucket: string): S3Client {
  return new S3Client({ bucket, ...credentials() });
}

/* READ */

const gone = (error: unknown) => {
  const it = error as { code?: string; name?: string };
  return it?.code === "NoSuchKey" || it?.code === "ERR_S3_FILE_NOT_FOUND" || it?.name === "NoSuchKey";
};

export async function getText(s3: S3Client, key: string): Promise<string | null> {
  try {
    return await s3.file(key).text();
  } catch (error) {
    if (gone(error)) return null;
    throw error;
  }
}

/* WRITE */

export async function putBytes(
  s3: S3Client,
  key: string,
  body: Uint8Array | string,
  opts: { type: string; cacheControl?: string },
): Promise<void> {
  const url = s3.presign(key, { method: "PUT", expiresIn: 900, type: opts.type });
  const headers: Record<string, string> = { "Content-Type": opts.type };
  if (opts.cacheControl) headers["Cache-Control"] = opts.cacheControl;
  const res = await fetch(url, { method: "PUT", body, headers });
  if (!res.ok) throw new Error(`s3: put ${key} failed ${res.status} ${(await res.text()).slice(0, 200)}`);
}

/* DELETE */

export async function del(s3: S3Client, keys: string[], batch = 32): Promise<number> {
  for (let i = 0; i < keys.length; i += batch) {
    await Promise.all(keys.slice(i, i + batch).map((key) => s3.delete(key)));
  }
  return keys.length;
}

/* LIST */

export async function list(s3: S3Client, prefix = ""): Promise<string[]> {
  const keys: string[] = [];
  let token: string | undefined;
  do {
    const page = await s3.list({ prefix, maxKeys: 1000, continuationToken: token });
    for (const item of page?.contents ?? []) keys.push(item.key);
    token = page?.isTruncated ? (page.nextContinuationToken ?? undefined) : undefined;
  } while (token);
  return keys;
}
