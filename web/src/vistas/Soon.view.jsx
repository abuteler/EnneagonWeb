import React from 'react';

import Grid from '@material-ui/core/Grid';
import { ReactComponent as LogoSoon } from '../assets/img/logo_landing.svg';
import { ReactComponent as EnneagonStudios } from '../assets/img/Typography9S.svg';
import './Soon.view.scss';
import { Typography } from '@material-ui/core';

class VistaSoon extends React.Component {
  
  render () {

    return (
      <Grid id="VistaSoon" container direction="column" justify="center" alignItems="center">
        <Grid item><LogoSoon className="EnneagonLogo" /></Grid>
        <Grid item>
          <EnneagonStudios className="EnneagonStudios" />
        </Grid>
        <Grid item>
          <Typography variant="h6" className="proximamente">Pr&oacute;ximamente :: Coming soon <span className="blinker">_</span></Typography>
        </Grid>
      </Grid>
    );
  }
}

export default VistaSoon;
