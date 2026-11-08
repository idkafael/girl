// Marcações de entrega dos roteiros (/roteiro/<modelo>).
// Guarda no Upstash Redis conectado ao projeto na Vercel:
//   hash "roteiro:<modelo>"  ->  { "<item>": "<data ISO da entrega>" }
// GET  /api/entregas?roteiro=jv               -> { ok, itens: {...} }
// POST /api/entregas  { roteiro, id, feito }  -> { ok, itens: {...} }

const URL_REDIS = process.env.KV_REST_API_URL || process.env.UPSTASH_REDIS_REST_URL;
const TOKEN = process.env.KV_REST_API_TOKEN || process.env.UPSTASH_REDIS_REST_TOKEN;
const NOME = /^[a-z0-9-]{1,40}$/;

async function redis(...cmd) {
  const r = await fetch(URL_REDIS, {
    method: "POST",
    headers: { Authorization: `Bearer ${TOKEN}`, "Content-Type": "application/json" },
    body: JSON.stringify(cmd),
  });
  if (!r.ok) throw new Error(`redis ${r.status}`);
  return (await r.json()).result;
}

async function ler(roteiro) {
  const lista = (await redis("HGETALL", `roteiro:${roteiro}`)) || [];
  const itens = {};
  for (let i = 0; i < lista.length; i += 2) itens[lista[i]] = lista[i + 1];
  return itens;
}

module.exports = async (req, res) => {
  res.setHeader("Cache-Control", "no-store");
  if (!URL_REDIS || !TOKEN) {
    return res.status(503).json({ ok: false, erro: "banco-nao-configurado" });
  }
  try {
    if (req.method === "GET") {
      const roteiro = String(req.query.roteiro || "");
      if (!NOME.test(roteiro)) return res.status(400).json({ ok: false, erro: "roteiro-invalido" });
      return res.json({ ok: true, itens: await ler(roteiro) });
    }
    if (req.method === "POST") {
      const { roteiro, id, feito } = req.body || {};
      if (!NOME.test(String(roteiro)) || !NOME.test(String(id))) {
        return res.status(400).json({ ok: false, erro: "dados-invalidos" });
      }
      if (feito) await redis("HSET", `roteiro:${roteiro}`, id, new Date().toISOString());
      else await redis("HDEL", `roteiro:${roteiro}`, id);
      return res.json({ ok: true, itens: await ler(roteiro) });
    }
    res.setHeader("Allow", "GET, POST");
    return res.status(405).json({ ok: false, erro: "metodo" });
  } catch (e) {
    return res.status(500).json({ ok: false, erro: "falha-no-banco" });
  }
};
