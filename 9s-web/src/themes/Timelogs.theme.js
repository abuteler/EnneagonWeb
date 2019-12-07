import { createMuiTheme, responsiveFontSizes } from '@material-ui/core/styles';
import { indigo, orange } from '@material-ui/core/colors';


let TimelogsTheme = createMuiTheme({
  palette: {
    primary: indigo,
    secondary: orange,
    background: {
      default: '#cecece',
    },
  },
  overrides: {
    MuiContainer: {
      root: {
        background: '#cecece',
        padding: 11,
      },
    },
    MuiCard: {
      root: {
        padding: 11,
        width: 180,
        position: 'relative',
      },
    },
    MuiCardContent: {
      root: {
        padding: 11,
      },
    },
    MuiCardActions: {
      root: {
        position: 'absolute',
        bottom: 0,
        right: 0,
        display: 'block',
        textAlign: 'right',
      },
    },
    // MuiIcon: {
    //   root: {}
    // }
    MuiIconButton: {
      root: {
        fontSize: '1.8rem',
      },
    },
  },
});

TimelogsTheme = responsiveFontSizes(TimelogsTheme);

export default TimelogsTheme;