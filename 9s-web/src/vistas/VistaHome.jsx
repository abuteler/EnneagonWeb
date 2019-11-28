import React from 'react';

import Grid from '@material-ui/core/Grid';

import Header from '../components/Header/Header';
import './VistaHome.scss';


class VistaHome extends React.Component {
  
  render () {

    return (
      <Grid id="VistaHome" container justify="center">
        <Grid item xs={12}>
          <Header />
        </Grid>
        <Grid item xs={12} className="InnerViewContainer">
          inner-views
        </Grid>
        <Grid item xs={12}>
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
