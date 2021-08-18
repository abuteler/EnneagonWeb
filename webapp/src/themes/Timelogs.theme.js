import { createTheme, responsiveFontSizes } from '@material-ui/core/styles';
import { indigo, orange, lightGreen } from '@material-ui/core/colors';


let TimelogsTheme = createTheme({
  palette: {
    primary: indigo,
    secondary: orange,
    background: {
      default: 'white',
    },
  },
  overrides: {
    MuiContainer: {
      root: {
        background: 'white',
        padding: '11px 11px !important',
      },
    },
    MuiTypography: {
      h3: {
        fontSize: '1.8rem !important',
        marginBottom: 20
      },
      h4: {
        fontSize: '1.6rem !important',
        marginTop: 20
      },
    },
    MuiCard: {
      root: {
        padding: 11,
        width: 180,
        position: 'relative',
        background: lightGreen[50],
      },
    },
    MuiCardContent: {
      root: {
        padding: 0,
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