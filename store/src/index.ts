import "reflect-metadata";
import {createConnection} from "typeorm";

async function main() {
  try {
    const conn =  await createConnection();
    await conn.close();
  } catch (err) {
    console.log(err);
  }  
}

main();
