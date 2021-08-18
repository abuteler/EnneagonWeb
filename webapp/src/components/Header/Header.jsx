import React from 'react';
import { NavLink } from "react-router-dom";

import Grid from '@material-ui/core/Grid';
import { AccountBox, AccessTime } from '@material-ui/icons';

import './Header.scss';
import { ReactComponent as LogoTop } from '../../assets/img/logo_top.svg';
import { ReactComponent as EnneagonStudios } from '../../assets/img/Typography9S.svg';

class Header extends React.Component {
  
  render () {

    return (
      <header id="Header">
        <Grid container justifyContent="center">
          <Grid item container xs={12} className="TopContainer" alignContent="center">
            <Grid item xs={1} xl={2}></Grid>
            <Grid item xs={10} xl={8} className="TipografiaLogo">
              <LogoTop className="EnneagonLogo"/>
              <EnneagonStudios className="EnneagonStudios" />
            </Grid>
            <Grid item xs={1} xl={2} className="UserAccessContainer">
              <NavLink exact={true} activeClassName='is-active' className="link" to="/">Clientes</NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/timelog"><AccessTime className="icon"/></NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/"><AccountBox className="icon"/></NavLink>
            </Grid>
          </Grid>
          <Grid item container xs={12} justifyContent="center" className="NavContainer">
            <Grid item lg={6}>
              <nav>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/home">Home</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/blog" >Blog</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/contacto">Contacto</NavLink>
              </nav>
            </Grid>
          </Grid>
        </Grid>
      </header>
    );
  }
}

export default Header;
