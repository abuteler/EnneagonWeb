import { GET_BLOGPOSTS, ADD_BLOGPOST, EDIT_BLOGPOST, DELETE_BLOGPOST, BLOGPOSTS_LOADING } from '../actions/types';

const initialState = {
  blogposts: [],
  loading: false
};

export default function (state = initialState, action) {
  switch(action.type) {
    case GET_BLOGPOSTS:
      return {
        ...state,
        blogposts: action.payload,
        loading: false
      }
    case ADD_BLOGPOST:
      return {
        ...state,
        blogposts: [action.payload, ...state.blogposts]
      }
    case EDIT_BLOGPOST:
      return {
        ...state,
        blogposts: [action.payload, ...state.blogposts] //?
      }
    case DELETE_BLOGPOST:
      return {
        ...state,
        blogposts: state.blogposts.filter(blogpost => blogpost._id !== action.payload)
      }
    case BLOGPOSTS_LOADING:
      return {
        ...state,
        loading: true
      }
    default:
      return state;
  };
}
