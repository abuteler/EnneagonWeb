import React from 'react';
import { NavLink } from "react-router-dom";

import { Grid, Hidden } from '@material-ui/core/';
import { AccountBox, AccessTime } from '@material-ui/icons';

import './Header.scss';
import { ReactComponent as LogoTop } from '../../assets/img/logo_top.svg';
import { ReactComponent as EnneagonStudiosLg } from '../../assets/img/Typography9S_lg.svg';
import { ReactComponent as EnneagonStudiosMd } from '../../assets/img/Typography9S_md.svg';
import { ReactComponent as EnneagonStudiosSm } from '../../assets/img/Typography9S_sm.svg';
import { ReactComponent as EnneagonStudiosSmSubtitle } from '../../assets/img/Typography9S_sm_subtitle.svg';

class Header extends React.Component {
  
  render () {

    return (
      <header>
        <Grid container justifyContent="center">
          <Grid item container className="TopContainer" alignContent="center">
            <Hidden xsDown><Grid item md={1} lg={2}></Grid></Hidden>
            <Grid item xs={12} lg={8} className="TipografiaLogo">
              <LogoTop className="EnneagonLogo"/>
              <Hidden smDown><EnneagonStudiosLg className="EnneagonStudios lg" /></Hidden>
              <Hidden xsDown mdUp><EnneagonStudiosMd className="EnneagonStudios md" /></Hidden>
              <Hidden smUp>
                <EnneagonStudiosSm className="EnneagonStudios sm" />
                <EnneagonStudiosSmSubtitle className="EnneagonStudiosSubtitle" />
              </Hidden>
            </Grid>
            <Grid item container xs={12} lg={2} className="UserMenu" justifyContent="flex-end" alignContent="flex-end" alignItems="flex-end">
              <NavLink exact={true} activeClassName='is-active' className="link" to="/">Clientes</NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/timelog"><AccessTime className="icon"/></NavLink>
              <NavLink exact={true} activeClassName='is-active' className="link" to="/"><AccountBox className="icon"/></NavLink>
            </Grid>
          </Grid>
          <Grid item container xs={12} justifyContent="center" className="NavContainer">
            <Grid item lg={6}>
              <nav>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/home">Home</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/blog" >About Us</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/blog" >Services</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/blog" >Technologies</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/blog" >Blog</NavLink>
                <NavLink exact={true} activeClassName='is-active' className="link" to="/contacto">Contact</NavLink>
              </nav>
            </Grid>
          </Grid>
        </Grid>
      </header>
    );
  }
}

export default Header;
