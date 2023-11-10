import { gql } from "graphql-tag";

const typeDefs = gql`
    type BlogPost {
        id: ID
        title: String
        body: String
        dateCreated: String
        dateModified: String
    }
    type BlogPostList {
        data: [BlogPost!]! # can return empty, but not null
        total: Int
    }

    input BlogPostInput {
        title: String!
        body: String!
    }

    type Query {
        "Get all blog posts"
        getAllBlogPosts: BlogPostList
    }

    type Mutation {
        createBlogPost(input: BlogPostInput): BlogPost
        updateBlogPost(input: BlogPostInput): BlogPost
        deleteBlogPost(id: ID!): String
    }
`;

export default typeDefs;