import "reflect-metadata";
import {
  registerEnumType,
  ObjectType,
  Field,
  Float,
  InputType,
  Resolver,
  Query,
  Arg,
  buildSchema,
} from "type-graphql";
import { v4 as UUID } from "uuid";
import { ApolloServer } from "apollo-server";

export enum EngineStatus {
  On,
  Off,
}

registerEnumType(EngineStatus, {
  name: "EngineStatus",
  description: "Current engine status",
});

@ObjectType()
export class Engine {
  @Field()
  code!: string;

  @Field((type) => EngineStatus)
  status!: EngineStatus;

  @Field((type) => Float)
  fuel!: number;
}

@InputType()
export class NewEngine {
  @Field((type) => EngineStatus)
  status!: EngineStatus;

  @Field((type) => Float)
  fuel!: number;
}

@Resolver(Engine)
export class EngineResolver {
  @Query((returns) => Engine)
  engine(@Arg("code") code: string): Engine {
    return {
      code: code,
      fuel: Math.random(),
      status: EngineStatus.On,
    };
  }

  registerEngine(@Arg("newEngine") newEngine: NewEngine): Engine {
    return {
      code: UUID(),
      status: newEngine.status || EngineStatus.Off,
      fuel: newEngine.fuel || Math.random(),
    };
  }
}

async function main() {
  const schema = await buildSchema({
    resolvers: [EngineResolver],
  });

  const server = new ApolloServer({ schema });

  const info = await server.listen(process.env.PORT);

  console.debug("Server running on %s", info.url);
}

main();
