import { mergeResolvers } from '@graphql-tools/merge'

import blog from './blog';

const allResolvers:any = [];

const modules = [blog];
modules.forEach(module => allResolvers.push(module));

const resolvers = mergeResolvers(allResolvers);

export default resolvers;