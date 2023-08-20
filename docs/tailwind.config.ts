import { gen_config } from "../tailwind.config";

const config = gen_config("..");
config.content.push(...config.content.map(p => "." + p));

export default config;
