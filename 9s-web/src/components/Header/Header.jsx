import React from 'react';
import { NavLink } from "react-router-dom";

import Grid from '@material-ui/core/Grid';
import { AccountBox, Home, Timelapse } from '@material-ui/icons';

import './Header.scss';
import { ReactComponent as LogoTop } from '../../assets/img/logo_top.svg';
import { ReactComponent as EnneagonStudios } from '../../assets/img/Typography9S.svg';

class Header extends React.Component {
  
  render () {

    return (
      <header id="Header">
        <Grid container justify="center">
          <Grid item container xs={12} className="TopContainer" alignContent="center">
            <Grid item xs={1} xl={2}></Grid>
            <Grid item xs={10} xl={8} className="TipografiaLogo">
              <EnneagonStudios className="EnneagonStudios" />
              <LogoTop className="EnneagonLogo"/>
            </Grid>
            <Grid item xs={1} xl={2} className="UserAccessContainer">
              <NavLink exact={true} activeClassName='is-active' className="link" to="/">Sign In</NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/timelog"><Timelapse className="icon"/></NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/"><AccountBox className="icon"/></NavLink>
            </Grid>
          </Grid>
          <Grid item container xs={12} justify="center" className="NavContainer">
            <Grid item lg={6}>
              <nav>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/home"><Home className="icon"/> Home</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/" >Blog</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/">Contacto</NavLink>
              </nav>
            </Grid>
          </Grid>
        </Grid>
      </header>
    );
  }
}

export default Header;
