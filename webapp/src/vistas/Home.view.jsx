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
        <h1>{t('welcome')}</h1>
        <h2>{t('who')}</h2>
        <p>{t('who_text')}</p>
        <section id="nearshore">
          <img src={NearshoreMap} alt="Nearshore map"/>
          <div className="nearshore-text">
            <h2>{t('where')}</h2>
            <p>{t('where_text')}</p>
            <h2>{t('why')}</h2>
            <p>{t('why_text')}</p>
            <p>
              En Enneagon Studios creemos que cualquiera sea el escenario y el campo de estudio o trabajo, realizar cambios positivos es posible cuando se agrupan y combinan los recursos disponibles con amor e inteligencia.
              Que crecer y brindar oportunidades de crecimiento a otras personas es viable, y necesario.<br />
              Que las oportunidades de cambio y mejora están presentes en todo momento cuando las personas involucradas ponen lo mejor de sí mismas para encontrarlas y aprovecharlas.<br />
              Que en el contexto de una crisis severa es posible hacer mejoras, y sin dudas es mejor hacer control de daños que rendirse y cruzarse de brazos a ver como la escena de la que somos parte termina en tragedia.
              Y que si aún con poco se puede lograr mucho, con la suma de un esfuerzo colectivo es posible crear y sostener un contexto favorable, en el que el crecimiento sea personal, social y global; saludable y sustentable.
              En Enneagon Studios no nos alcanza con buscar el propio crecimiento: creemos y queremos firmemente que un mundo mejor es y siga siendo posible.<br />
            </p>
          </div>
        </section>
      </Container>
    );
  }
}

export default withTranslation()(VistaHome);
