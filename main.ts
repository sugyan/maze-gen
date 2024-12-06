import { Hono } from "https://deno.land/x/hono@v4.3.11/mod.ts";
import { generate_wasm } from "./maze-generator/pkg/maze_generator.js";

const app = new Hono();

app.get("/api/image", (c) => {
  const w = parseInt(c.req.query("w") || "20");
  const h = parseInt(c.req.query("h") || "20");
  if (isNaN(w) || isNaN(h) || w > 250 || h > 250) {
    c.status(400);
    return c.body("Invalid width or height");
  }
  c.header("Content-Type", "image/png");
  return c.body(generate_wasm(w, h));
});

Deno.serve(app.fetch);
