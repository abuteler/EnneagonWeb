import React from 'react';
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import { ThemeProvider } from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Grid from '@material-ui/core/Grid';

import EnneaTheme from './Theme';
import VistaLanding from './vistas/VistaLanding';
import VistaHome from './vistas/VistaHome';

import './App.scss';


function App() {
  const basePath = '';
  return (
    <ThemeProvider theme={EnneaTheme}>
      <CssBaseline />
      <div className="App">
        <Grid container justify="center" direction="column" className="full-height">
          <Grid item container className="full-height">
            <section id="App_Vistas">
            <Router>
              <Switch>
                <Route exact path={`${basePath}/`} component={VistaLanding} />
                <Route exact path={`${basePath}/home`} component={VistaHome} />
                {/* <Route exact path={`${basePath}/blog`} component={VistaUnica} /> */}
                {/* <Route exact path={`${basePath}/contacto`} component={VistaUnica} /> */}
              </Switch>
            </Router>
            </section>
          </Grid>
        </Grid>
      </div>
    </ThemeProvider>
  );
}

export default App;
