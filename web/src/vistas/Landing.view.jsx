import React from 'react';
import { NavLink } from "react-router-dom";

import Grid from '@material-ui/core/Grid';
import { Laptop, Tablet, Smartphone, PresentToAll } from '@material-ui/icons';
import { ReactComponent as LogoLanding } from '../assets/img/logo_landing.svg';
import './Landing.view.scss';

class VistaLanding extends React.Component {
  
  render () {

    return (
      <Grid id="VistaLanding" container direction="column" justify="center" alignItems="center">
        <Grid item className="landing_logo"><LogoLanding /></Grid>
        <Grid item className="landing_icons-container">
          <Laptop className="device laptop" />
          <Tablet className="device tablet" />
          <Smartphone className="device smartphone" />
          <NavLink exact={true} activeClassName='is-active' className="link" to="/soon">
            <div className="arrow-container">
              <PresentToAll className="arrow" />
            </div>
          </NavLink>
          <div className="blinker">_</div>
        </Grid>
      </Grid>
    );
  }
}

export default VistaLanding;