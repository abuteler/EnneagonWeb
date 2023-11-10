const fooPost = { title: "foo", body: "bar", dateCreated: "10/11/2023" };

const blog = {
  Query: {
    getAllBlogPosts: () => ({ data: [fooPost], total: 1})
  },
  Mutation: {
    createBlogPost: () => fooPost,
  }
};

export default blog;
