import React from 'react';
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import { Spanish, English } from './translations';

import { Provider } from 'react-redux';
import store from './store';

import { ThemeProvider } from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';

import Header from './components/Header/Header';
import Footer from './components/Footer/Footer';
import MainTheme from './themes/Main.theme';
import VistaLanding from './vistas/Landing.view';
import VistaSoon from './vistas/Soon.view';
import VistaHome from './vistas/Home.view';
import VistaBlog from './vistas/Blog.view';
import VistaContacto from './vistas/Contacto.view';
import VistaTimeLogs from './vistas/TimeLogs.view';

import './App.scss';

i18n
  .use(initReactI18next)
  .init({
    resources: {
      en: { translation: English },
      es: { translation: Spanish },
    },
    language: 'en',
    fallbackLng: 'en'
  })

const handleRenderInnerView = (InnerComponent) => {
  // See React HOCs for reference.
  return class extends React.Component {
    // 2do: handle user sessions
    render() {
      return (
        <div className="InnerViewRootContainer">
          <Header />
          <section className="InnerViewContentContainer">
            <InnerComponent />
          </section>
          <Footer />
        </div>
      );
    };
  };
};

function App() {
  const basePath = '';
  return (
    <ThemeProvider theme={MainTheme}>
      <CssBaseline />
      <Provider store={store}>
        <div className="App">
          <Router>
            <Switch>
              <Route exact path={`${basePath}/`} component={VistaLanding} />
              <Route exact path={`${basePath}/soon`} component={VistaSoon} />
              <Route exact path={`${basePath}/home`} component={handleRenderInnerView(VistaHome)} />
              <Route exact path={`${basePath}/timelog`} component={handleRenderInnerView(VistaTimeLogs)} />
              <Route exact path={`${basePath}/blog`} component={handleRenderInnerView(VistaBlog)} />
              <Route exact path={`${basePath}/contacto`} component={handleRenderInnerView(VistaContacto)} />
              {/* STRIPPED COMPONENTS */}
              <Route exact path={`${basePath}/stripped/timelog`} component={VistaTimeLogs} />
            </Switch>
          </Router>
        </div>
      </Provider>
    </ThemeProvider>
  );
}

export default App;
