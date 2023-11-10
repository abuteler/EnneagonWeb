import { mergeTypeDefs } from "@graphql-tools/merge";

import blog from './blog';

const allTypeDefs: any = [];

const modules = [blog];
modules.forEach(module => allTypeDefs.push(module));

const typeDefs = mergeTypeDefs(allTypeDefs);

export default typeDefs;