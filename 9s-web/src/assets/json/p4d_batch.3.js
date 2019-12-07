/* Week template
[
  {
    "week_day": "lunes",
    "date": "dd MMM",
    "year": "2019",
    "time_entries": [{
      "in": "",
      "out": "",
      "tasks": []
    }]
  },
  {
    "week_day": "martes",
    "date": "dd MMM",
    "year": "2019",
    "time_entries": [{
      "in": "",
      "out": "",
      "tasks": []
    }]
  },
  {
    "week_day": "miercoles",
    "date": "dd MMM",
    "year": "2019",
    "time_entries": [{
      "in": "",
      "out": "",
      "tasks": []
    }]
  },
  {
    "week_day": "jueves",
    "date": "dd MMM",
    "year": "2019",
    "time_entries": [{
      "in": "",
      "out": "",
      "tasks": []
    }]
  },
  {
    "week_day": "viernes",
    "date": "dd MMM",
    "year": "2019",
    "time_entries": [{
      "in": "",
      "out": "",
      "tasks": []
    }]
  }
];
*/

export const p4dLog = {
  "projectName": "MAE",
  "weeks": [
    [
      {
        "week_day": "lunes",
        "date": "02 Dec",
        "year": "2019",
        "time_entries": [{
          "in": "00:00",
          "out": "02:00",
          "tasks": ["(x Viernes previo de 19 a 21hs) relevamiento y ajustes via documento QA `Prueba Nueva WEB - Consolidado`."]
        }]
      },
      {
        "week_day": "martes",
        "date": "03 Dec",
        "year": "2019",
        "time_entries": [{
          "in": "14:00",
          "out": "21:00",
          "tasks": [
            "Responsividad -> Home: correcciones en template y estilos de componentes de boletin diario y mesa de entradas.",
            "Responsividad -> Home: correcciones en template y estilos de vista de novedades.",
            "Responsividad -> Home: componente de Mercado.",
          ]
        }]
      },
      {
        "week_day": "miercoles",
        "date": "04 Dec",
        "year": "2019",
        "time_entries": [{
          "in": "13:10",
          "out": "16:50",
          "tasks": [
            "Responsividad -> Home: correcciones en template y estilos de componente de licitaciones.",
          ]
        }, {
          "in": "18:10",
          "out": "20:10",
          "tasks": [
            "Responsividad -> Home: finalizacion de ajustes en componente de licitaciones.",
            "Responsividad -> Common: Restauración del hack de Ale solo para resoluciones superiores lg y mejorando el selector para corregir un error por el que se producia un comportamiento no deseado con los paddings de columnas siblings.",
          ]
        }]
      },
      {
        "week_day": "jueves",
        "date": "05 Dec",
        "year": "2019",
        "time_entries": [{
          "in": "11:00",
          "out": "11:30",
          "tasks": ["Shadowing a Nacho remoto para que pudiera encarar el nuevo cuadro en Negociacion Secundaria, y modificar el resto."]
        }, {
          "in": "13:30",
          "out": "19:30",
          "tasks": [
            "Ajustes finales sobre el laburo de Nacho en modificaciones a twigs de Vistas de Cuadros.",
            "Responsividad -> MAE Today.",
            "Resolucion de conflictos entre master y dev para habilitar merge de PR",
          ]
        }]
      },
      {
        "week_day": "viernes",
        "date": "dd MMM",
        "year": "2019",
        "time_entries": [{
          "in": "",
          "out": "",
          "tasks": []
        }]
      }
    ]
  ]
};
  