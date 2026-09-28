// Anécdotas en español — traducción fiel de `fr.rs` (texto de referencia).

pub const TEXTES: &[(&str, &str)] = &[
    // ─────────────────────────────── France ───────────────────────────────
    ("SS_MALADIE",
        "La Seguridad Social francesa nace de las ordenanzas del 4 y el 19 de octubre de 1945, en la estela del \
        programa del Consejo Nacional de la Resistencia, «Les Jours heureux» («Los días felices», marzo de \
        1944). El alto funcionario Pierre Laroque diseña su arquitectura; el ministro comunista de Trabajo \
        Ambroise Croizat instala sus cajas a marchas forzadas en 1946. El proyecto de una caja única choca \
        con los médicos liberales, la Mutualité, los regímenes ya existentes y los cuadros, que se niegan a \
        quedar integrados: de ahí el mosaico de regímenes que subsiste hoy. El verdadero primer hito es \
        anterior: las leyes sobre los seguros sociales de 1928 y 1930."),
    ("SS_VIEILLESSE_PLAF",
        "La primera ley sobre las pensiones obreras y campesinas (5 de abril de 1910) fijaba la edad de \
        jubilación en 65 años, cuando pocos obreros la alcanzaban: la CGT la combatió como «la jubilación \
        para los muertos». El reparto se impone en 1941, bajo Vichy, con la asignación a los trabajadores \
        asalariados mayores, y 1945 lo generaliza. Los 60 años llegan con la ordenanza del 26 de marzo de \
        1982. Desde entonces, cada reforma ha movido un cursor: Balladur (1993, cálculo sobre los 25 mejores \
        años en lugar de 10), Fillon (2003), Woerth (2010, 62 años), Touraine (2014, duración de \
        cotización), Borne (2023, 64 años, aprobada mediante el 49.3 tras meses de manifestaciones)."),
    ("SS_VIEILLESSE_DEPLAF",
        "Tocar las pensiones ha seguido siendo la reforma más inflamable del país. En noviembre-diciembre de \
        1995, el plan Juppé sobre los regímenes especiales provoca tres semanas de huelgas en los transportes, \
        y el gobierno retira su parte sobre las pensiones."),
    ("FAMILLE",
        "Las prestaciones familiares nacieron de iniciativas patronales: durante la Gran Guerra, algunos \
        industriales pagan un «sobresueldo familiar» y luego se organizan en cajas de compensación para que \
        ningún empleador salga perjudicado por contratar a padres de familia. La ley Landry del 11 de marzo de \
        1932 hace obligatoria la afiliación en la industria y el comercio; el Código de la familia de 1939 \
        endurece la orientación natalista. La universalidad, principio de 1945, se rompió en 2015 con la \
        modulación de las prestaciones según los ingresos, en contra de la opinión de las asociaciones \
        familiares."),
    ("AT_MP",
        "Antes de 1898, el obrero herido debía probar la culpa de su patrón ante el juez: es decir, casi \
        nunca. Hicieron falta dieciocho años de idas y venidas parlamentarias, desde la propuesta presentada \
        en 1880 por Martin Nadaud, antiguo albañil de la Creuse convertido en diputado, para arrancar el \
        compromiso de la ley del 9 de abril de 1898: indemnización automática, pero a tanto alzado. La ley \
        del 25 de octubre de 1919 la extiende a las primeras enfermedades profesionales, como el saturnismo. \
        El escándalo del amianto, prohibido en Francia el 1 de enero de 1997, llevó a crear en 2000 un fondo \
        de indemnización específico, el FIVA."),
    ("CSG_DEDUCTIBLE",
        "La CSG es obra del primer ministro Michel Rocard, en la ley de presupuestos para 1991: 1,1 % sobre \
        casi todos los ingresos, incluidos los del capital. El texto se aprueba mediante el 49.3 y la moción \
        de censura que sigue se queda a solo cinco votos de la mayoría. ¿Impuesto o cotización? El debate \
        jurídico duró años; el Tribunal de Justicia europeo acabó tratándola como una cotización social. \
        Nacida modesta, hoy recauda más que el impuesto sobre la renta."),
    ("CSG_NON_DEDUCTIBLE",
        "La subida de 1,7 puntos de 2018, que financiaba la supresión de las cotizaciones salariales de \
        enfermedad y desempleo, beneficiaba a los trabajadores pero golpeaba a los jubilados sin \
        contrapartida. Su enfado alimentó el movimiento de los chalecos amarillos: ya en diciembre de 2018, \
        el gobierno crea un tipo intermedio para proteger las pensiones modestas."),
    ("CRDS",
        "Creada por la ordenanza del 24 de enero de 1996 (plan Juppé) para alimentar la CADES, la caja \
        encargada de amortizar la deuda social, la CRDS debía extinguirse en 2009. Cada nueva oleada de \
        déficit, y en último lugar la deuda ligada a la Covid, ha retrasado el plazo, hoy fijado en 2033: el \
        impuesto «temporal» ha superado los treinta años."),
    ("CHOMAGE",
        "La Unédic nace de un acuerdo firmado el 31 de diciembre de 1958 entre patronal y sindicatos, a \
        impulso del general de Gaulle, que prefiere un régimen paritario a una gestión estatal. André \
        Bergeron, líder de Force ouvrière, hará de ella durante décadas el símbolo del paritarismo. El \
        equilibrio se ha invertido desde 2018: suprimida la cotización salarial, el Estado fija ahora el \
        marco de las negociaciones mediante una carta de encuadre, y ha retomado el control por decreto \
        cuando los interlocutores sociales no se ponían de acuerdo."),
    ("AGIRC_ARRCO_T1",
        "En 1946-1947 los cuadros se negaron a quedar integrados en el régimen general: el convenio colectivo \
        del 14 de marzo de 1947 les da su propia caja por puntos, la AGIRC. Los no cuadros obtienen el \
        equivalente con el acuerdo del 8 de diciembre de 1961, que funda la ARRCO. Ambas se fusionan el \
        1 de enero de 2019 (acuerdo del 30 de octubre de 2015). El mismo acuerdo había creado un «malus» del \
        10 % durante tres años para quien se jubilara a la edad legal; muy impopular, se suprimió en 2023."),
    ("AGIRC_ARRCO_T2",
        "Hasta 2018, la condición de cuadro se leía directamente en la pensión: cotizaciones AGIRC, garantía \
        mínima de puntos, caja propia. La fusión borró esos rasgos distintivos, y la propia definición de \
        cuadro tuvo que renegociarse en un acuerdo interprofesional en 2020."),
    ("AGIRC_ARRCO_CEG_T1",
        "En 2019 la CEG tomó el relevo de la AGFF, creada a su vez en 2001 por una razón precisa: cuando llega \
        la jubilación a los 60 años en 1982, los regímenes complementarios no siguen. Hace falta una \
        estructura específica (la ASF en 1983, luego la AGFF) para financiar el pago de las pensiones \
        complementarias sin reducción entre los 60 y los 65 años."),
    ("PREVOYANCE_CADRE_MIN",
        "Esta obligación del 1,50 % se remonta al artículo 7 del convenio colectivo de los cuadros del 14 de \
        marzo de 1947: un seguro de fallecimiento financiado por el empleador, contrapartida del apego de los \
        cuadros a su propio régimen. Cuando la AGIRC desapareció en 2019, la obligación estuvo a punto de \
        desaparecer con ella; el acuerdo interprofesional del 17 de noviembre de 2017 la mantuvo."),
    ("REDUCTION_FILLON",
        "Las bonificaciones de cotizaciones sobre los salarios bajos empiezan con Édouard Balladur en 1993, se \
        amplían con la «rebaja» de Juppé y luego con las ayudas ligadas a las 35 horas de Martine Aubry. La \
        ley del 17 de enero de 2003, impulsada por François Fillon, ministro de Asuntos Sociales, las fusiona \
        en un único dispositivo: de ahí el apodo que le ha quedado. Su reverso se conoce como la «trampa de \
        los salarios bajos»: cada euro de aumento cerca del SMIC (salario mínimo) hace perder al empleador \
        parte de la bonificación. El informe Bozio-Wasmer (2024) lo cuantificó e inspiró la reforma de \
        2026."),
    ("ALSACE_MOSELLE_MALADIE",
        "Cuando Alsacia y Mosela son alemanas, de 1871 a 1918, reciben los seguros sociales de Bismarck: \
        enfermedad (1883), accidentes (1884), vejez (1889). De vuelta a Francia, sus habitantes se niegan a \
        perder derechos más avanzados que los de la «Francia del interior»; la ley del 1 de junio de 1924 \
        mantiene el derecho local. El régimen complementario de enfermedad es su heredero directo, como los \
        dos días festivos adicionales (Viernes Santo y 26 de diciembre)."),
    ("AIDE_POSTE_EA",
        "La ley del 10 de julio de 1987 obliga a las empresas de 20 o más trabajadores a emplear al menos un \
        6 % de trabajadores con discapacidad, so pena de contribución. La ley del 11 de febrero de 2005, la \
        gran ley sobre la discapacidad, transforma los antiguos «talleres protegidos» en empresas adaptadas: \
        empresas del mercado ordinario, sometidas al derecho laboral, y no ya estructuras médico-sociales."),
    ("REDUC_SAL_HS",
        "«Trabajar más para ganar más»: el lema de campaña de Nicolas Sarkozy se convierte en la ley TEPA de \
        agosto de 2007, que exime las horas extraordinarias de impuestos y cotizaciones. François Hollande \
        suprime lo esencial del dispositivo en 2012; Emmanuel Macron lo restablece, adelantado al 1 de enero \
        de 2019 bajo la presión de los chalecos amarillos. Tres cambios de rumbo en doce años para la misma \
        línea de la nómina."),
    ("DFP_HS",
        "La deducción a tanto alzado patronal es una supervivencia de la ley TEPA de 2007: cuando la reforma de \
        2012 suprimió las exenciones, se conservó para las empresas de menos de 20 trabajadores. La ley de \
        presupuestos rectificativa del verano de 2022 la extendió a las empresas de 20 a 249 trabajadores, \
        con un importe menor."),
    ("FPT_CNRACL",
        "La CNRACL se creó en 1945 y sigue gestionada por la Caisse des dépôts, desde Burdeos. Durante mucho \
        tiempo excedentaria gracias a una demografía favorable, tuvo que ceder durante décadas parte de sus \
        recursos a otros regímenes en concepto de «compensación demográfica». El envejecimiento de sus \
        propios afiliados la hizo caer en déficit, de ahí las subidas del tipo patronal que soportan las \
        entidades locales y los hospitales."),

    // ──────────────────────────────── Suisse ───────────────────────────────
    ("CH_AVS",
        "El AVS (seguro de vejez y supervivencia) ya figuraba entre las reivindicaciones de la huelga general \
        de noviembre de 1918. Un artículo constitucional se aprueba en 1925, pero la primera ley se rechaza \
        en 1931. Hay que esperar a la votación del 6 de julio de 1947: alrededor del 80 % de síes, con una \
        participación récord. Más recientemente, el pueblo ha elevado la edad de jubilación de las mujeres a \
        65 años (AVS 21, septiembre de 2022, por estrecha mayoría) y aceptado en marzo de 2024 una 13.ª \
        pensión, la primera iniciativa de izquierdas sobre los seguros sociales que supera el listón."),
    ("CH_AI",
        "Inscrito en la Constitución ya en 1925 junto con el AVS, el seguro de invalidez no entra en vigor \
        hasta 1960. Ante el aumento del número de pensiones, la 5.ª revisión (2008) erigió en principio que \
        «la readaptación prima sobre la pensión»: el AI financia primero la vuelta al trabajo."),
    ("CH_APG",
        "Las APG (compensación por pérdida de ingresos) nacieron en 1940 para compensar la pérdida de salario \
        de los soldados movilizados. Este régimen sirvió de vehículo al seguro de maternidad, rechazado \
        varias veces por el pueblo (1984, 1987, 1999) antes de ser aceptado en septiembre de 2004. El \
        permiso de paternidad de dos semanas siguió el mismo camino, aprobado en votación en septiembre de \
        2020."),
    ("CH_AC",
        "Hasta mediados de los años setenta, el seguro de desempleo era voluntario en Suiza. La crisis del \
        petróleo y la ola de despidos que siguió llevaron a hacerlo obligatorio: artículo constitucional de \
        1976, ley (LACI) de 1982."),
    ("CH_LPP",
        "El sistema de los «tres pilares» se inscribió en la Constitución con la votación de diciembre de \
        1972, preferido a una iniciativa del Partido del Trabajo que quería una pensión popular única y \
        generosa. Después hicieron falta más de diez años para que la ley LPP entrara en vigor, en 1985."),
    ("CH_AAP",
        "La ley sobre el seguro de enfermedad y accidentes se aprueba en votación en febrero de 1912. Crea la \
        SUVA, establecimiento público con sede en Lucerna, que desde 1918 asegura los accidentes en la \
        industria y en los oficios de riesgo."),
    ("CH_AANP",
        "Particularidad suiza: el accidente ocurrido fuera del trabajo, esquiando o haciendo bricolaje, está \
        cubierto por el seguro del trabajador. Heredada de la ley de 1911 y de la SUVA, esta cobertura se \
        generalizó a todos los sectores con la ley sobre el seguro de accidentes de 1981."),
    ("CH_IS",
        "En Ginebra, el impuesto en origen de los trabajadores fronterizos franceses está en el centro de un \
        acuerdo del 29 de enero de 1973: el cantón grava los salarios en el lugar y abona a los departamentos \
        de Ain y Alta Saboya una compensación calculada sobre la masa salarial de los fronterizos. Otros \
        ocho cantones se rigen por un acuerdo de 1983, según el cual el fronterizo tributa en Francia."),

    // ────────────────────────────── Luxembourg ─────────────────────────────
    ("LU_AM",
        "Luxemburgo adopta el seguro de enfermedad obligatorio de los obreros ya en 1901, según el modelo de \
        Bismarck. Durante más de un siglo, obreros y empleados privados pertenecen a cajas distintas; el \
        «estatuto único» del 1 de enero de 2009 los reúne en una sola Caja Nacional de Salud."),
    ("LU_ME",
        "La Mutualidad de los empleadores nació con el estatuto único de 2009: al suprimir la distinción entre \
        obreros y empleados, la reforma generalizó el mantenimiento del salario en caso de enfermedad. Para \
        que las pequeñas empresas no soporten solas el coste, los empleadores lo mutualizan."),
    ("LU_AP",
        "Como sus vecinos, Luxemburgo construyó sus pensiones según el ejemplo alemán desde principios del \
        siglo XX. El país ha alimentado durante mucho tiempo una reserva considerable, gestionada por un \
        fondo de compensación, gracias a la alta proporción de trabajadores fronterizos que cotizan sin ser \
        aún pensionistas."),

    // ────────────────────────────── Allemagne ──────────────────────────────
    ("DE_KRANKENVERSICHERUNG",
        "La ley de 1883 sobre el seguro de enfermedad de los obreros es el primero de los seguros sociales de \
        Bismarck. El canciller lo anuncia en el mensaje imperial del 17 de noviembre de 1881; el cálculo es \
        político: tras haber prohibido las organizaciones socialistas (ley de 1878), quiere apartar a los \
        obreros de la socialdemocracia ofreciéndoles una protección procedente del Estado."),
    ("DE_UNFALLVERSICHERUNG",
        "Segundo pilar bismarckiano, el seguro de accidentes data de 1884. Las Berufsgenossenschaften, \
        mutuas de empleadores por rama que todavía lo gestionan, son casi tan antiguas como el propio \
        régimen."),
    ("DE_RENTENVERSICHERUNG",
        "El seguro de invalidez y vejez de 1889 fijaba la edad de la pensión en 70 años. La gran reforma de \
        1957, bajo Adenauer, indexa las pensiones a los salarios. En 1986, el ministro de Trabajo Norbert \
        Blüm pega carteles con «Die Rente ist sicher» («la pensión está asegurada»); la frase se ha \
        convertido en Alemania en el ejemplo por excelencia de la promesa política ridiculizada."),
    ("DE_ARBEITSLOSENVERSICHERUNG",
        "El seguro de desempleo alemán nace en 1927, bajo la República de Weimar. Tres años después, un \
        desacuerdo sobre su financiación hace caer la última gran coalición de Weimar, en marzo de 1930: una \
        disputa por unas décimas de punto de cotización, en el umbral de la crisis que se llevará por delante \
        a la República."),
    ("DE_PFLEGEVERSICHERUNG",
        "Quinta rama de la seguridad social, el seguro de dependencia es creado en 1995 por Norbert Blüm. Para \
        compensar el coste para los empleadores se suprime un día festivo: el Buß- und Bettag (Día de \
        penitencia y oración). Solo Sajonia lo conservó, y a cambio sus trabajadores pagan una parte mayor de \
        la cotización. El recargo para las personas sin hijos se deriva de una sentencia del Tribunal \
        Constitucional de 2001."),
    ("DE_LOHNSTEUER",
        "La retención del impuesto sobre el salario por el empleador data de la reforma financiera de \
        Matthias Erzberger de 1920, que centraliza el impuesto sobre la renta en el Reich. Erzberger, ya \
        odiado por la derecha nacionalista por haber firmado el armisticio de 1918, es asesinado en 1921."),
    ("DE_SOLI",
        "Instaurado en 1991 para financiar la reunificación bajo el canciller Kohl, el Soli debía ser \
        provisional. Suprimido en 2021 para cerca del 90 % de los contribuyentes, subsiste para las rentas \
        más altas; el Tribunal Constitucional dictaminó en marzo de 2025 que su mantenimiento seguía siendo \
        conforme a la Ley Fundamental."),
    ("DE_KIRCHENSTEUER",
        "El impuesto eclesiástico es la contrapartida histórica de las secularizaciones de 1803, cuando los \
        bienes de la Iglesia pasaron a los príncipes. La Constitución de Weimar (1919) lo garantiza, y la Ley \
        Fundamental de 1949 retomó el artículo. El Estado lo recauda con el impuesto sobre el salario; se \
        escapa de él abandonando oficialmente la propia Iglesia, paso que dan cientos de miles de alemanes \
        cada año."),

    // ──────────────────────────────── Autriche ─────────────────────────────
    ("AT_SV",
        "Austria-Hungría adopta el seguro de accidentes (1887) y el seguro de enfermedad (1888) siguiendo la \
        estela de Alemania. El derecho actual se basa en la ASVG, la ley general de seguridad social de 1955, \
        el mismo año en que el país recupera su plena soberanía."),
    ("AT_LOHNSTEUER",
        "Particularidad austriaca: las pagas 13.ª y 14.ª, abonadas en verano y a final de año, se benefician de \
        un tipo impositivo reducido a tanto alzado. Esta ventaja, anclada desde hace tiempo en los convenios \
        colectivos, es una de las conquistas más defendidas del país."),

    // ───────────────────────────────── Italie ──────────────────────────────
    ("IT_IVS",
        "El antepasado del INPS es una caja nacional de previsión creada en 1898, de adhesión voluntaria. La \
        reforma Dini de 1995 hace pasar a Italia a un cálculo «contributivo», basado en las cotizaciones \
        pagadas. En diciembre de 2011, en plena crisis de la deuda, la ministra Elsa Fornero anuncia el \
        aumento de la edad y la congelación de la indexación de las pensiones y rompe a llorar en plena \
        rueda de prensa: la imagen dio la vuelta al país."),
    ("IT_TFR",
        "El TFR sustituyó en 1982 a la antigua indemnización por antigüedad. Este salario diferido, \
        provisionado por el empleador, sirvió durante mucho tiempo de financiación barata a las empresas \
        italianas. Desde 2007, el trabajador que no dice nada ve su TFR destinado a un fondo de pensiones \
        («quien calla, otorga»); muchos han optado por dejarlo en la empresa."),
    ("IT_IRPEF",
        "El IRPEF nace de la gran reforma fiscal de 1973-1974. En su creación contaba con 32 tramos, del 10 % \
        al 72 %. Las sucesivas reformas solo han dejado unos pocos; reducir el número de tramos, o incluso \
        instaurar un «impuesto plano», se ha convertido en una seña política de la derecha italiana."),
    ("IT_ADD_REG",
        "El recargo regional se crea en 1997, junto con el IRAP, por el ministro de Finanzas Vincenzo Visco, en \
        el marco del «federalismo fiscal»: las regiones financian su sistema sanitario con un impuesto cuyo \
        tipo fijan ellas mismas. Resultado: el mismo salario no tributa igual en Milán que en Nápoles."),
    ("IT_INAIL",
        "La ley del 17 de marzo de 1898 hace obligatorio el seguro de los obreros de la industria contra los \
        accidentes, el mismo año que la ley francesa. El INAIL, instituto único, se crea en 1933."),
    ("IT_NASPI",
        "La NASpI es una de las piezas de la Jobs Act de Matteo Renzi (2015), que también flexibilizó la \
        protección contra los despidos prevista por el famoso artículo 18 del Estatuto de los Trabajadores \
        de 1970, a costa de una ruptura duradera con la CGIL."),
    ("IT_MATERNITA",
        "La ley 1204 de 1971 sobre la protección de las madres trabajadoras fue una de las grandes conquistas \
        de los años de movilización social que siguieron al «otoño caliente» de 1969. El permiso de \
        paternidad obligatorio no apareció hasta 2012, por un solo día."),
    ("IT_BONUS_CUNEO",
        "La «cuña fiscal» (cuneo fiscale), la diferencia entre lo que cuesta un trabajador y lo que cobra, es \
        una obsesión italiana. El «bono de 80 euros» de Matteo Renzi (2014) abrió una serie de dispositivos \
        que cada gobierno ha rebautizado y prorrogado: Draghi y luego Meloni lo ampliaron y cambiaron su \
        forma."),
    ("IT_ESONERO",
        "La exención de cotizaciones a cargo del trabajador fue creada por el gobierno Draghi en 2022 frente a \
        la inflación, y luego ampliada por el gobierno Meloni. Renovada año tras año, se transformó a partir \
        de 2025 en una ventaja fiscal."),
    ("IT_FONDO_GARANZIA",
        "El fondo de garantía del TFR se creó con la misma ley de 1982 que el propio TFR: sin él, un salario \
        diferido durante años podía desaparecer con la quiebra del empleador."),

    // ──────────────────────────────── Espagne ──────────────────────────────
    ("ES_CC",
        "La primera ley social española es la ley Dato de 1900 sobre accidentes de trabajo. El Instituto \
        Nacional de Previsión se funda en 1908; pero la Seguridad Social moderna no nace hasta la ley de \
        bases de 1963, en vigor desde 1967, bajo el franquismo. En 1995, el «Pacto de Toledo» compromete a \
        todos los partidos a sacar las pensiones de la batalla electoral."),
    ("ES_MEI",
        "El mecanismo de equidad intergeneracional es obra del ministro José Luis Escrivá (2021-2023). \
        Sustituye al «factor de sostenibilidad» de la reforma de Rajoy de 2013, que habría reducido las \
        pensiones con el aumento de la esperanza de vida y nunca llegó a aplicarse."),
    ("ES_FOGASA",
        "El FOGASA se crea en 1976, durante la Transición, para garantizar a los trabajadores el pago de sus \
        salarios en caso de insolvencia del empleador."),
    ("ES_DESEMPLEO",
        "La reforma laboral de 2021, negociada por la ministra Yolanda Díaz con sindicatos y patronal, hizo del \
        contrato indefinido la regla en un país que fue durante mucho tiempo el campeón europeo de los \
        contratos temporales. Las cotizaciones por desempleo más altas en los contratos temporales son uno \
        de sus instrumentos."),

    // ──────────────────────────────── Portugal ─────────────────────────────
    ("PT_SS",
        "Bajo el Estado Novo de Salazar, la previsión se organiza en cajas corporativas por profesión (ley de \
        1935). Tras la Revolución de los Claveles del 25 de abril de 1974, esas cajas se unifican en una \
        seguridad social universal, que consagra la Constitución de 1976."),
    ("PT_IRS",
        "El IRS entró en vigor el 1 de enero de 1989 y sustituyó un mosaico de impuestos cedulares. Desde \
        entonces, Portugal ha multiplicado los regímenes especiales, como el de «residentes no habituales» \
        (2009), que atraía a jubilados y directivos extranjeros antes de cerrarse a los recién llegados en \
        2024."),
    ("PT_FCT",
        "El Fondo de Compensación del Trabajo y su fondo de garantía se crearon en 2013, bajo el programa de \
        asistencia de la «troika» (FMI, BCE, Comisión Europea), como contrapartida de la reducción de las \
        indemnizaciones por despido."),

    // ──────────────────────────────── Belgique ─────────────────────────────
    ("BE_ONSS_SAL",
        "La seguridad social belga nace de un «pacto social» negociado en secreto durante la Ocupación entre \
        empresarios y sindicalistas. El decreto-ley del 28 de diciembre de 1944 crea la ONSS, que desde \
        entonces recauda todas las cotizaciones en un único lugar."),
    ("BE_ONSS_PAT",
        "La concertación social belga descansa desde 1944 en la idea de que empresarios y sindicatos gestionan \
        juntos la seguridad social. La indexación automática de los salarios, poco frecuente en Europa, es \
        otro de sus pilares: los empleadores la cuestionan regularmente en nombre de la competitividad."),
    ("BE_PP",
        "La reforma fiscal de 1962 instituye el impuesto sobre las personas físicas y la retención profesional \
        practicada por el empleador. Bélgica figura desde hace tiempo entre los países de la OCDE donde el \
        trabajo está más gravado: cada gobierno anuncia un «tax shift» para corregir este rasgo."),
    ("BE_BONUS_EMPLOI",
        "El bono al empleo, creado en 2005, responde a un problema preciso: con un salario bajo, la ganancia \
        neta de volver a trabajar en lugar de cobrar una prestación era a veces casi nula. Se la llama la \
        «trampa del empleo»."),
    ("BE_RED_STRUCT",
        "La reducción estructural nació en 2004 de la fusión de varias bonificaciones de cotizaciones \
        patronales. Es el equivalente belga de la reducción Fillon francesa, con la misma lógica \
        decreciente."),

    // ──────────────────────────────── Royaume-Uni ──────────────────────────
    ("UK_NI_SAL",
        "La National Insurance nace de la National Insurance Act de 1911, impulsada por David Lloyd George: \
        vende la reforma con un lema que se hizo célebre, «ninepence for fourpence» (nueve peniques de \
        prestaciones por cuatro de cotización). El informe Beveridge de 1942 y la ley de 1946 del gobierno \
        Attlee la convierten en la base del Estado del bienestar británico."),
    ("UK_NI_PAT",
        "La subida de la National Insurance patronal al 15 % a partir de abril de 2025, anunciada en el primer \
        presupuesto de Rachel Reeves, primera mujer al frente de la Hacienda británica (Chancellor of the \
        Exchequer), fue la medida de ese presupuesto más contestada por las empresas británicas."),
    ("UK_INCOME_TAX",
        "El impuesto sobre la renta británico fue inventado por William Pitt el Joven en 1799 para financiar la \
        guerra contra la Francia revolucionaria. Abolido en 1816, restablecido en 1842 por Robert Peel, \
        siempre debía ser temporal. La retención en origen, el PAYE, se introduce en 1944."),

    // ──────────────────────────────── Irlande ──────────────────────────────
    ("IE_USC",
        "La Universal Social Charge se introdujo en el presupuesto de 2011, en plena crisis bancaria irlandesa \
        y del rescate europeo, en sustitución de dos gravámenes anteriores. Pensada como medida de \
        urgencia, se quedó."),
    ("IE_PRSI",
        "El PRSI actual data de 1979. La subida progresiva de sus tipos, decidida a partir de 2024, financia \
        las pensiones ante el envejecimiento de la población, tras el abandono, bajo la presión popular, del \
        proyecto de elevar la edad de la pensión a 67 años."),

    // ──────────────────────────────── Pays-Bas ─────────────────────────────
    ("NL_LOONHEFFING",
        "La pensión básica AOW, cuyas cotizaciones están incluidas en la loonheffing, fue instaurada en 1957 \
        por el primer ministro Willem Drees. Generaciones de jubilados han dicho «trekken van Drees» («cobrar \
        de Drees») para hablar de su pensión."),
    ("NL_ZVW",
        "La ley del seguro de salud de 2006, impulsada por el ministro Hans Hoogervorst, suprimió la distinción \
        entre cajas públicas y seguro privado: todos los residentes suscriben un seguro básico con \
        aseguradoras privadas en competencia, un modelo único en Europa."),
    ("NL_AOF",
        "La antigua ley de invalidez, la WAO de 1967, fue víctima de su éxito: a principios de los años \
        noventa, casi un millón de neerlandeses la percibían, y el primer ministro Ruud Lubbers hablaba de un \
        país «enfermo». La WIA la sustituyó en 2006 poniendo el acento en la capacidad de trabajo restante."),

    // ─────────────────────────────── Scandinavie ───────────────────────────
    ("SE_SKATT",
        "En 1976, Astrid Lindgren, la creadora de Pippi Calzaslargas, descubre que las normas fiscales la \
        llevan a un tipo marginal superior al 100 %. Publica un cuento satírico, «Pomperipossa en el país de \
        Monismania». El debate que desata contribuye ese mismo año a la derrota de los socialdemócratas, en \
        el poder desde hacía 44 años."),
    ("SE_ARBETSGIVARAVGIFT",
        "La reforma de las pensiones de 1994-1999, votada por cinco partidos, creó un sistema de cuentas \
        nocionales imitado en varios países. Cada año, los suecos reciben un sobre naranja que resume sus \
        derechos: «orange kuvertet» se ha convertido en un símbolo nacional."),
    ("DK_AM",
        "El arbejdsmarkedsbidrag (contribución al mercado laboral) fue creado en 1994 por la reforma fiscal del \
        gobierno socialdemócrata de Poul Nyrup Rasmussen. Dinamarca financia lo esencial de su protección \
        social con impuestos en lugar de cotizaciones, lo que explica uno de los niveles de presión sobre la \
        renta más altos del mundo."),
    ("DK_ATP",
        "La ATP, pensión complementaria obligatoria, se instauró en 1964. Su cotización a tanto alzado, y no \
        proporcional al salario, la convierte en una curiosidad entre los regímenes de pensiones europeos."),
    ("FI_TYEL",
        "Finlandia instauró en 1962 la pensión de los asalariados del sector privado, gestionada por \
        aseguradoras privadas con mandato público: un modelo original de gestión descentralizada de un \
        régimen obligatorio."),

    // ──────────────────────────────── Pays baltes ──────────────────────────
    ("EE_TULUMAKS",
        "En 1994, bajo el joven primer ministro Mart Laar, Estonia se convierte en uno de los primeros países \
        de Europa en adoptar un impuesto sobre la renta de tipo único. El ejemplo inspiró a toda Europa \
        central y oriental en los años 2000."),
    ("EE_KOGUMISPENSION",
        "Obligatorio para las generaciones jóvenes desde 2002, el segundo pilar pasó a ser voluntario en 2021 a \
        iniciativa del partido Isamaa. Decenas de miles de estonios lo abandonaron para recuperar sus \
        ahorros."),
    ("EE_SOTSIAALMAKS",
        "El impuesto social estonio del 33 % lo paga íntegramente el empleador y financia a la vez la pensión \
        y la sanidad. El país optó por un gravamen único y legible en lugar de una serie de cotizaciones."),
    ("LT_SODRA",
        "En 2019, Lituania trasladó casi todas las cotizaciones patronales al trabajador, elevando al mismo \
        tiempo los salarios brutos casi un 29 % para compensar: el salario neto no cambiaba, pero la nómina \
        hacía visible el coste real de la protección social."),
    ("LV_IIN",
        "Letonia abandonó en 2018 su impuesto de tipo único en favor de una escala progresiva, a contracorriente \
        de la tendencia que había marcado la región desde los años noventa."),

    // ──────────────────────────── Europe centrale ──────────────────────────
    ("PL_EMERYTALNE",
        "La reforma de 1999 creó fondos de pensiones privados obligatorios, los OFE. En 2014, el gobierno de \
        Donald Tusk transfiere al seguro público ZUS cerca de la mitad de sus activos, para aligerar la \
        deuda pública: una de las marchas atrás más espectaculares en materia de pensiones de capitalización \
        en Europa."),
    ("PL_ZDROWOTNE",
        "El «Polski Ład» (Nuevo Orden Polaco), reforma fiscal de 2022 del gobierno del PiS, suprimió la \
        deducibilidad de la cotización sanitaria en el impuesto. Su caótica entrada en vigor obligó a \
        corregir de urgencia nóminas de enero de 2022 en las que algunos salarios netos habían bajado."),
    ("CZ_DAN",
        "Hasta 2020, el impuesto checo se calculaba sobre un «salario superbruto», que sumaba al bruto las \
        cotizaciones patronales. Esta curiosidad, introducida en 2008, se suprimió en 2021."),
    ("SK_DAN",
        "En 2004, la Eslovaquia de Ivan Mikloš adopta un tipo único del 19 % sobre la renta, las sociedades y \
        el IVA, y se convierte en el escaparate europeo del «impuesto plano». El gobierno de Fico reintroduce \
        un tramo del 25 % en 2013."),
    ("HU_SZJA",
        "La Hungría de Viktor Orbán instauró en 2011 un impuesto sobre la renta de tipo único del 16 %, rebajado \
        al 15 % en 2016. Por política natalista, las madres de cuatro hijos están exentas desde 2020; los \
        menores de 25 años lo están desde 2022, dentro de cierto límite."),
    ("HU_SZOCHO",
        "La contribución social patronal húngara se ha rebajado paso a paso, del 27 % en 2016 al 13 % en 2022, \
        en el marco de acuerdos salariales con los interlocutores sociales: menos cargas a cambio de un \
        salario mínimo más alto."),
    ("RO_CAS",
        "En 2018, Rumanía trasladó casi todas las cotizaciones sociales del empleador al trabajador, exigiendo \
        que los salarios brutos se elevaran en la misma medida. La medida, apodada «revolución fiscal», \
        explica por qué el trabajador rumano soporta lo esencial de las cotizaciones en su nómina."),
    ("RO_IMPOZIT",
        "Rumanía adoptó un impuesto de tipo único en 2005 (16 %) y luego lo rebajó al 10 % en 2018, uno de \
        los tipos más bajos de la Unión Europea."),
    ("BG_DANAK",
        "Con su impuesto de tipo único del 10 %, instaurado en 2008, Bulgaria aplica uno de los impuestos sobre \
        la renta más bajos de la Unión Europea. El 1 de enero de 2026 adoptó el euro, lo que obligó a \
        convertir todos los topes de cotización."),
    ("HR_POREZ",
        "La reforma fiscal de 2024 suprimió el «prirez», recargo municipal sobre el impuesto sobre la renta, y \
        dejó a las ciudades la tarea de fijar ellas mismas sus tipos del impuesto sobre la renta dentro de una \
        horquilla legal."),
    ("GR_EFKA",
        "El EFKA se creó en 2017 para reunir una multitud de cajas profesionales, entre ellas el IKA de los \
        asalariados del sector privado. Durante la crisis de la deuda soberana, las pensiones griegas se \
        redujeron en numerosas ocasiones en aplicación de los memorandos firmados con los acreedores."),
    ("CY_GESY",
        "Chipre no tuvo un sistema sanitario universal hasta 2019, con el lanzamiento del GESY, esperado desde \
        una ley de 2001. Hasta entonces, gran parte de la atención se pagaba directamente o mediante seguro \
        privado."),

    // ───────────────────────────── Micro-États ─────────────────────────────
    ("AD_IRPF",
        "Andorra no conoció ningún impuesto sobre la renta de las personas físicas hasta 2015. Su \
        introducción, con un tipo máximo del 10 %, forma parte de los compromisos asumidos por el principado \
        para salir de las listas de paraísos fiscales y negociar con la Unión Europea."),
    ("MC_CAR",
        "Mónaco no percibe impuesto sobre la renta desde que el príncipe Carlos III lo abolió en 1869, gracias \
        a los ingresos del casino. Solo los franceses no se libran: tras la crisis de 1962, durante la cual \
        el general de Gaulle hace instalar controles aduaneros en la frontera, el convenio fiscal de 1963 los \
        somete al impuesto francés."),

    // ──────────────────────────────── Amérique du Nord ─────────────────────
    ("US_SS",
        "La Social Security Act es firmada por Franklin D. Roosevelt el 14 de agosto de 1935, en plena Gran \
        Depresión. La primera pensionista, Ida May Fuller, maestra de Vermont, cotizó menos de 25 dólares; \
        vivió hasta los 100 años y cobró cerca de 23 000 dólares de pensión."),
    ("US_MEDICARE",
        "Medicare es creado en 1965 por Lyndon B. Johnson en el marco de la «Great Society». Firma la ley en \
        Independence (Misuri), en presencia del expresidente Harry Truman, cuyo proyecto de seguro de salud \
        había fracasado: Truman recibe la primera tarjeta de Medicare."),
    ("US_ADD_MEDICARE",
        "El recargo del 0,9 % sobre las rentas altas fue introducido en 2013 por la Affordable Care Act, el \
        «Obamacare», para financiar la reforma del seguro de salud."),
    ("US_FUTA",
        "El desempleo federal forma parte de la Social Security Act de 1935. Su mecanismo de crédito, que \
        reduce un tipo del 6 % al 0,6 % para los empleadores que cotizan a un régimen estatal, se concibió \
        para empujar a cada estado a crear su propio seguro de desempleo."),
    ("US_IMPOT_FED",
        "El impuesto federal sobre la renta solo fue posible gracias a la 16.ª enmienda de 1913, después de que \
        el Tribunal Supremo lo hubiera declarado inconstitucional en 1895. La retención en origen se instaura \
        en 1943, para financiar la guerra, a partir de una idea del economista Beardsley Ruml."),
    ("US_IMPOT_STATE",
        "El recargo californiano del 1 % por encima de un millón de dólares de renta fue creado por la \
        Proposition 63, aprobada en referéndum en 2004 para financiar los servicios de salud mental. \
        California aplica así el tipo marginal del impuesto estatal más alto de Estados Unidos."),
    ("US_CA_SDI",
        "California fue en 2004 el primer estado estadounidense en instaurar un permiso familiar retribuido, \
        financiado por esta cotización. Estados Unidos sigue siendo el único país rico sin permiso de \
        maternidad retribuido a nivel federal."),
    ("CA_RPC2",
        "La mejora del régimen de pensiones de Canadá se acordó en 2016 entre Ottawa y las provincias, primera \
        ampliación importante desde su creación; se aplica progresivamente desde 2019, y Quebec mejoró su \
        propio régimen con el mismo calendario."),
    ("ON_IMPOT_PROV",
        "A diferencia de Quebec, Ontario confía la recaudación de su impuesto sobre la renta a la Agencia \
        Tributaria de Canadá, en virtud de un acuerdo de recaudación: el trabajador de Ontario presenta una \
        única declaración."),
    ("CA_RPC",
        "El Régimen de Pensiones de Canadá se crea en 1965 bajo el gobierno de Lester B. Pearson. El Quebec de \
        Jean Lesage se niega a adherirse y crea su propio régimen; las reservas quebequesas financian la \
        Caisse de dépôt et placement du Québec, que se ha convertido en uno de los mayores inversores \
        institucionales del país."),
    ("CA_AE",
        "Una primera ley federal sobre el seguro de desempleo, aprobada en 1935 por el gobierno de Bennett, es \
        anulada por los tribunales en nombre del reparto de competencias. Hubo que modificar la Constitución \
        en 1940 para que el seguro federal de desempleo viera la luz."),
    ("CA_IMPOT_FED",
        "El impuesto federal sobre la renta se introduce en 1917 con la Income War Tax Act (ley del impuesto de \
        guerra sobre la renta), presentada como una medida temporal para financiar la Primera Guerra \
        Mundial."),
    ("QC_RRQ",
        "El RRQ es fruto de la «Revolución Tranquila»: en 1964-1965, el gobierno de Jean Lesage negocia con \
        Ottawa el derecho a tener su propio régimen. Las cotizaciones acumuladas dan origen, en 1965, a la \
        Caisse de dépôt et placement du Québec."),
    ("QC_RQAP",
        "Quebec obtuvo la gestión de sus propias prestaciones parentales mediante un acuerdo con Ottawa en \
        2005, tras un largo pulso; sin embargo, ese mismo año el Tribunal Supremo reconocía la competencia \
        federal sobre esas prestaciones. El RQAP, lanzado en 2006, creó semanas reservadas a los padres, que \
        aumentaron con fuerza su uso del permiso."),
    ("QC_IMPOT_PROV",
        "Quebec es la única provincia que recauda por sí misma su impuesto sobre la renta. Maurice Duplessis \
        crea este impuesto provincial en 1954, para afirmar la autonomía fiscal de Quebec frente a Ottawa."),
    ("QC_FSS",
        "El seguro de enfermedad quebequés entra en vigor en noviembre de 1970. Provoca una huelga de los \
        médicos especialistas en octubre de 1970, en plena Crisis de Octubre, a la que la Asamblea Nacional \
        pone fin con una ley especial."),
    ("MX_IMSS",
        "El Instituto Mexicano del Seguro Social se crea en 1943 bajo el presidente Manuel Ávila Camacho. \
        Cerca de la mitad de los trabajadores mexicanos, empleados en la economía informal, siguen hoy fuera \
        de su cobertura."),
    ("MX_INFONAVIT",
        "El INFONAVIT, fondo de vivienda de los trabajadores, se crea en 1972 bajo el presidente Luis \
        Echeverría. Se ha convertido en el mayor prestamista hipotecario de América Latina."),
    ("MX_RETIRO",
        "México creó en 1992 un sistema de ahorro individual para el retiro y, en 1997, las Afores, gestoras \
        privadas de las cuentas de los trabajadores, según el modelo chileno."),

    // ────────────────────────────── Amérique du Sud ─────────────────────────
    ("BR_INSS",
        "La previsión social brasileña data de la ley Eloy Chaves de 1923, que crea una caja de jubilación para \
        los ferroviarios. El INSS, instituto único, se crea en 1990."),
    ("BR_FGTS",
        "El FGTS se crea en 1966, bajo el régimen militar, para sustituir la estabilidad en el empleo \
        garantizada a los trabajadores tras diez años de antigüedad. Los empleadores ganaban libertad para \
        despedir; los trabajadores, un capital movilizable para comprar una vivienda."),

    // ───────────────────────────────── Asie ────────────────────────────────
    ("JP_KENPO",
        "La ley japonesa del seguro de enfermedad de los asalariados se aprueba en 1922 y se aplica desde 1927. \
        El seguro de enfermedad universal, extendido a toda la población, se logra en 1961."),
    ("JP_KOSEI",
        "El seguro de pensiones de los asalariados nace en 1942, durante la guerra. En 2007 se descubre que \
        unos 50 millones de expedientes de cotización no pueden asignarse a nadie: el escándalo de las \
        «pensiones desaparecidas» contribuye a la derrota del primer gobierno de Shinzō Abe en las \
        elecciones a la Cámara Alta de 2007."),
    ("JP_ROUSAI",
        "El seguro japonés de accidentes de trabajo se crea en 1947, el mismo año que la ley de normas \
        laborales, en el marco de las reformas de la posguerra."),
    ("JP_KAIGO",
        "El seguro de cuidados de larga duración, en vigor desde 2000, fue una de las respuestas del país al \
        envejecimiento más rápido del mundo. Se cotiza a partir de los 40 años."),
    ("JP_KOYO",
        "El seguro de empleo de 1974 sustituyó al seguro de desempleo creado en 1947. También financia ayudas \
        al mantenimiento del empleo, herencia de una cultura del empleo de por vida."),
    ("JP_SHOTOKUZEI",
        "Desde 2013 y hasta 2037, el impuesto sobre la renta lleva un recargo especial de reconstrucción del \
        2,1 %, que financia la reconstrucción tras el terremoto y el tsunami del 11 de marzo de 2011."),
    ("JP_JUMINZEI",
        "El impuesto de residencia se calcula sobre los ingresos del año anterior. Los recién titulados no lo \
        pagan, por tanto, durante su primer año de trabajo, y descubren en junio de su segundo año una bajada \
        de su salario neto."),
    ("CN_GONGJIJIN",
        "El fondo de vivienda se inspira en el Central Provident Fund de Singapur. Shanghái lo experimenta en \
        1991, en el marco de la reforma que pone fin a la vivienda asignada por la unidad de trabajo."),
    ("CN_IIT",
        "El impuesto sobre la renta chino se instaura en 1980 con un umbral de 800 yuanes al mes, que en la \
        práctica lo reservaba a los extranjeros. La reforma de 2018 eleva el umbral a 5000 yuanes e \
        introduce deducciones por la educación de los hijos, la vivienda o los padres mayores."),
    ("CN_YANGLAO",
        "La reforma de los años noventa combina un fondo común y cuentas individuales. En 2024, China emprendió \
        el primer aumento progresivo de su edad de jubilación desde los años cincuenta."),
    ("KR_NPS",
        "La pensión nacional coreana nace en 1988. Ante el agotamiento previsto de sus reservas, el Parlamento \
        aprobó en marzo de 2025 una reforma que lleva progresivamente el tipo de cotización del 9 % al 13 %, \
        primera subida desde 1998."),
    ("KR_NHI",
        "Corea del Sur logró la cobertura sanitaria universal en 1989, solo doce años después de la creación \
        del seguro de enfermedad obligatorio para las grandes empresas, en 1977."),
    ("KR_EI",
        "El seguro de empleo coreano se instaura en 1995. La crisis financiera asiática de 1997-1998, que \
        dispara el desempleo, lleva a extenderlo de urgencia a todas las empresas."),
    ("KR_SANJAE",
        "El seguro de accidentes de trabajo, creado en 1964, es el primer seguro social de la historia de \
        Corea del Sur."),
    ("KR_LTC",
        "El seguro de cuidados de larga duración se introdujo en 2008. Corea del Sur ha vivido uno de los \
        envejecimientos más rápidos de la OCDE, con la tasa de fecundidad más baja del mundo."),
    ("IN_EPF",
        "El fondo de previsión de los asalariados se crea en 1952. Solo cubre el sector formal, mientras que la \
        gran mayoría de los trabajadores indios pertenece a la economía informal."),
    ("IN_ESI",
        "La Employees' State Insurance Act se aprueba en 1948, solo un año después de la independencia: una de \
        las primeras leyes sociales de la India independiente."),
    ("IN_IMPOT",
        "El presupuesto de 2020 introdujo un «nuevo régimen» del impuesto, con tipos más bajos pero sin la \
        mayoría de las deducciones. Convertido en el régimen por defecto en 2023, deja al trabajador la \
        elección cada año."),
    ("AE_EXPAT",
        "Los Emiratos no tienen impuesto sobre la renta. Introdujeron el IVA en 2018 y un impuesto de \
        sociedades del 9 % en 2023, manteniendo la ausencia de impuesto sobre los salarios."),

    // ──────────────────────────────── Océanie ──────────────────────────────
    ("AU_SUPER",
        "La Superannuation Guarantee se introduce en 1992 bajo el primer ministro Paul Keating. La jubilación \
        obligatoria por capitalización ha convertido a los fondos de pensiones australianos en una de las \
        mayores reservas de ahorro para la jubilación del mundo."),
    ("AU_MEDICARE",
        "El seguro de enfermedad universal australiano nació dos veces: Medibank, creado en 1975 por el \
        gobierno de Whitlam y luego desmantelado por su sucesor, y Medicare, restablecido en 1984 por Bob \
        Hawke y financiado por esta contribución."),
    ("AU_INCOME_TAX",
        "En 1942, durante la guerra, el gobierno federal australiano retira a los estados el derecho a \
        recaudar el impuesto sobre la renta, «tomándolo prestado» mientras durara el conflicto. Nunca se lo \
        devolvió."),
    ("NZ_ACC",
        "El informe del juez Owen Woodhouse (1967) desemboca en 1974 en un sistema único en el mundo: toda \
        víctima de un accidente es indemnizada sin culpa, pero a cambio renuncia al derecho de demandar al \
        responsable."),
    ("NZ_KIWISAVER_EMP",
        "KiwiSaver es lanzado en 2007 por el ministro de Finanzas laborista Michael Cullen. La adhesión es \
        automática para los nuevos asalariados, que pueden salirse: un ejemplo a menudo citado de «empujón» \
        (nudge) en economía conductual."),
];
