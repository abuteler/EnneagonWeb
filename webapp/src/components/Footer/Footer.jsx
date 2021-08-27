import React from 'react';
import './Footer.scss'
class Footer extends React.Component {
  render () {
    return (
      <footer>
        <span>9S Footer</span>
        <a href="http://www.enneagonstudios.com/" target="_blank" rel="noopener noreferrer">
          {/* <img src={footer_logo} alt="Enneagon Studios Logo" /> */} A link
        </a>
      </footer>
    )
  }
}

export default Footer;
