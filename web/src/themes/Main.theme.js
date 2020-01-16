import { createMuiTheme, responsiveFontSizes } from '@material-ui/core/styles';
// import { lightGreen, indigo } from '@material-ui/core/colors';


let MainTheme = createMuiTheme({
  palette: {
    primary: {
      light: '#DDFED5',
      main: '#A3E1C2',
      dark: '#79C39E',
      contrastText: '#2f4f4f', //darkSlateGray
    },
    secondary: {
      main: '#6979A7',
    },
    background: {
      default: '#fff',
    },
  },
  typography: {
    fontSize: 16,
  },
  status: {
    danger: 'orange',
  },
});

MainTheme = responsiveFontSizes(MainTheme);

export default MainTheme;