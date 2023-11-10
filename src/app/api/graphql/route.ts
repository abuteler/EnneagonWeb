import { startServerAndCreateNextHandler } from "@as-integrations/next";
import { ApolloServer } from "@apollo/server";
import { NextRequest } from "next/server";
import resolvers from "./resolvers";
import typeDefs from "./schema";

const server = new ApolloServer({
    typeDefs,
    resolvers,
});

const handler = startServerAndCreateNextHandler<NextRequest>(server as any, {
    context: async req => ({ req }),
});

export { handler as GET, handler as POST };