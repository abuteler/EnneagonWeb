import React from 'react';

import Container from '@material-ui/core/Container';

import NearshoreMap from '../assets/img/9s-nearshore-americas.png';

import './Home.view.scss';
import { withTranslation } from 'react-i18next';


class VistaHome extends React.Component {
  render () {

    const { t } = this.props;
    return (
      <Container id="VistaHome" maxWidth="xl">
        <section id="nearshore">
          <div className="who">
            <h1>{t('home.welcome')}</h1>
            <h2>{t('home.who')}</h2>
            <p>{t('home.who_text')}</p>
          </div>
          <img src={NearshoreMap} alt="Nearshore map"/>
          <div className="map-text">
            <h2>{t('home.where')}</h2>
            <p>{t('home.where_text')}</p>
            <h2>{t('home.why')}</h2>
            <p>{t('home.why_text')}</p>
            <div className="key-values">
              <h3>{t('home.accountability')}</h3>
              <p>{t('home.why_text_2')}</p>
              <h3>{t('home.high_standards')}</h3>
              <p>{t('home.why_text_3')}</p>
              <h3>{t('home.low_cost')}</h3>
              <p>{t('home.why_text_4')}</p>
            </div>
          </div>
        </section>
      </Container>
    );
  }
}

export default withTranslation()(VistaHome);
