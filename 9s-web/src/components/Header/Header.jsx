import React from 'react';
import { NavLink } from "react-router-dom";

import Grid from '@material-ui/core/Grid';
import { AccountBox, Home } from '@material-ui/icons';

import './Header.scss';
import { ReactComponent as LogoTop } from '../../assets/img/logo_top.svg';

class VistaHome extends React.Component {
  
  render () {

    return (
      <header id="Header">
        <Grid container justify="center">
          <Grid item container xs={12} className="TopContainer">
            <Grid item xs={2}></Grid>
            <Grid item xs={8}>
              <h1>9S Home header</h1>
              <h2>bla</h2>
              <LogoTop />
            </Grid>
            <Grid item xs={2} className="UserAccessContainer">
              <NavLink exact={true} activeClassName='is-active' className="link" to="/timelog"><AccountBox className="icon"/></NavLink>
            </Grid>
          </Grid>
          <Grid item container xs={12} justify="center" className="NavContainer">
            <Grid item lg={6}>
              <nav>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/home"><Home className="icon"/> Home</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/contacto">Contacto</NavLink>
                {/* <NavLink exact={true} activeClassName='is-active' className="link" to="/blog">Blog</NavLink> */}
              </nav>
            </Grid>
          </Grid>
        </Grid>
      </header>
    );
  }
}

export default VistaHome;
