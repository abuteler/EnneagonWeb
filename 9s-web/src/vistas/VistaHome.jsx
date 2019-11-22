import React from 'react';
import { NavLink } from "react-router-dom";

import Grid from '@material-ui/core/Grid';

// import './VistaHome.scss';


class VistaHome extends React.Component {
  
  render () {

    return (
      <Grid container justify="center" direction="column">
        <Grid item>
          <header>
            <h1>9S Home header</h1>
            <hr />
            <h2>bla</h2>
          </header>
          <nav>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/home">Home</NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/contacto">Contacto</NavLink>
              {/* <NavLink exact={true} activeClassName='is-active' className="link" to="/blog">Blog</NavLink> */}
          </nav>
        </Grid>
        <Grid item>
          <footer>
            <span>bla </span>
            <a href="http://www.enneagonstudios.com/" target="_blank" rel="noopener noreferrer">
              {/* <img src={footer_logo} alt="Enneagon Studios Logo" /> */} bla
            </a>
          </footer>
        </Grid>
      </Grid>
    );
  }
}

export default VistaHome;
