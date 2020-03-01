import axios from 'axios';
import { GET_BLOGPOSTS, ADD_BLOGPOST, EDIT_BLOGPOST, DELETE_BLOGPOST, BLOGPOSTS_LOADING } from './types';

// const initialState = {
//   blogposts: [],
// };
const base_url = '/api/blogposts';

export const setBlogpostsLoading = () => {
  return {
    type: BLOGPOSTS_LOADING,
  };
};

export const getBlogposts = () => dispatch => {
  dispatch(setBlogpostsLoading());
  axios
    .get(base_url)
    .then(res=> dispatch({
      type: GET_BLOGPOSTS,
      payload: res.data
    }));
};

export const addBlogpost = blogpost => dispatch => {
  axios
    .post(base_url, blogpost)
    .then(res =>
      dispatch({
        type: ADD_BLOGPOST,
        payload: res.data
      })
    );
};

export const editBlogpost = (blogpost) => dispatch => {
  axios
    .put(`base_url/${blogpost._id}`, blogpost)
    .then(res =>
      dispatch({
        type: EDIT_BLOGPOST,
        payload: blogpost
      })
    )
};

export const deleteBlogpost = id => dispatch => {
  axios
    .delete(`base_url/${id}`)
    .then(res =>
      dispatch({
        type: DELETE_BLOGPOST,
        payload: id
      })
    )
};
