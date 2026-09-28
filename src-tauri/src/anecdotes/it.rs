// Aneddoti in italiano — traduzione fedele di `fr.rs` (testo di riferimento).

pub const TEXTES: &[(&str, &str)] = &[
    // ─────────────────────────────── France ───────────────────────────────
    ("SS_MALADIE",
        "La Sicurezza sociale francese nasce dalle ordinanze del 4 e 19 ottobre 1945, sulla scia del programma \
        del Consiglio nazionale della Resistenza, «Les Jours heureux» («I giorni felici», marzo 1944). L'alto \
        funzionario Pierre Laroque ne disegna l'architettura; il ministro comunista del Lavoro Ambroise \
        Croizat ne insedia le casse a marce forzate nel 1946. Il progetto di una cassa unica si scontra con i \
        medici liberi professionisti, la Mutualité, i regimi già esistenti e i quadri, che rifiutano di \
        esservi assorbiti: da qui il mosaico di regimi che sopravvive ancora oggi. La vera prima tappa è \
        precedente: le leggi sulle assicurazioni sociali del 1928 e del 1930."),
    ("SS_VIEILLESSE_PLAF",
        "La prima legge sulle pensioni operaie e contadine (5 aprile 1910) fissava l'età pensionabile a 65 \
        anni, quando pochi operai la raggiungevano: la CGT la combatté come «la pensione per i morti». La \
        ripartizione si impone nel 1941, sotto Vichy, con l'assegno ai vecchi lavoratori dipendenti, poi il \
        1945 la generalizza. I 60 anni arrivano con l'ordinanza del 26 marzo 1982. Da allora ogni riforma ha \
        spostato un cursore: Balladur (1993, calcolo sui 25 anni migliori invece di 10), Fillon (2003), \
        Woerth (2010, 62 anni), Touraine (2014, durata contributiva), Borne (2023, 64 anni, adottata con il \
        49.3 dopo mesi di manifestazioni)."),
    ("SS_VIEILLESSE_DEPLAF",
        "Toccare le pensioni è rimasta la riforma più incendiaria del paese. Nel novembre-dicembre 1995 il \
        piano Juppé sui regimi speciali provoca tre settimane di scioperi dei trasporti, e il governo ritira \
        la parte sulle pensioni."),
    ("FAMILLE",
        "Gli assegni familiari sono nati da iniziative padronali: durante la Grande Guerra alcuni industriali \
        versano un «supplemento familiare» sul salario, poi si organizzano in casse di compensazione perché \
        nessun datore di lavoro sia penalizzato dall'assumere padri di famiglia. La legge Landry dell'11 marzo \
        1932 rende obbligatoria l'adesione nell'industria e nel commercio; il Codice della famiglia del 1939 \
        accentua l'orientamento natalista. L'universalità, principio del 1945, è stata rotta nel 2015 con la \
        modulazione degli assegni in base al reddito, contro il parere delle associazioni familiari."),
    ("AT_MP",
        "Prima del 1898 l'operaio ferito doveva provare davanti al giudice la colpa del padrone: vale a dire \
        quasi mai. Ci vollero diciotto anni di navetta parlamentare, dalla proposta presentata nel 1880 da \
        Martin Nadaud, ex muratore della Creuse diventato deputato, per strappare il compromesso della legge \
        del 9 aprile 1898: risarcimento automatico, ma forfettario. La legge del 25 ottobre 1919 lo estende \
        alle prime malattie professionali, come il saturnismo. Lo scandalo dell'amianto, vietato in Francia \
        il 1° gennaio 1997, ha portato a creare nel 2000 un fondo di indennizzo dedicato, il FIVA."),
    ("CSG_DEDUCTIBLE",
        "La CSG è opera del primo ministro Michel Rocard, nella legge finanziaria per il 1991: 1,1% su quasi \
        tutti i redditi, compresi quelli da capitale. Il testo passa con il 49.3 e la mozione di censura che \
        segue manca la maggioranza di soli cinque voti. Imposta o contributo? Il dibattito giuridico è \
        durato anni; la Corte di giustizia europea ha finito per trattarla come un contributo sociale. Nata \
        modesta, oggi rende più dell'imposta sul reddito."),
    ("CSG_NON_DEDUCTIBLE",
        "L'aumento di 1,7 punti del 2018, che finanziava la soppressione dei contributi salariali per malattia \
        e disoccupazione, avvantaggiava i lavoratori ma colpiva i pensionati senza contropartita. La loro \
        rabbia ha alimentato il movimento dei gilet gialli: già nel dicembre 2018 il governo crea un'aliquota \
        intermedia per risparmiare le pensioni modeste."),
    ("CRDS",
        "Creata dall'ordinanza del 24 gennaio 1996 (piano Juppé) per alimentare la CADES, la cassa incaricata \
        di ammortizzare il debito sociale, la CRDS doveva estinguersi nel 2009. Ogni nuova ondata di deficit, \
        e da ultimo il debito legato al Covid, ha spostato la scadenza, oggi fissata al 2033: il prelievo \
        «temporaneo» ha superato i trent'anni."),
    ("CHOMAGE",
        "L'Unédic nasce da un accordo firmato il 31 dicembre 1958 tra padronato e sindacati, su impulso del \
        generale de Gaulle, che preferisce un regime paritetico a una gestione statale. André Bergeron, \
        leader di Force ouvrière, ne farà per decenni il simbolo del paritarismo. L'equilibrio si è \
        rovesciato dal 2018: soppresso il contributo salariale, lo Stato fissa ormai il quadro dei negoziati \
        con una lettera di indirizzo, e ha ripreso il controllo per decreto quando le parti sociali non si \
        sono accordate."),
    ("AGIRC_ARRCO_T1",
        "Nel 1946-1947 i quadri rifiutarono di essere assorbiti nel regime generale: il contratto collettivo \
        del 14 marzo 1947 dà loro una propria cassa a punti, l'AGIRC. I non quadri ottengono l'equivalente \
        con l'accordo dell'8 dicembre 1961, che fonda l'ARRCO. Le due si fondono il 1° gennaio 2019 (accordo \
        del 30 ottobre 2015). Lo stesso accordo aveva creato un «malus» del 10% per tre anni per chi andava \
        in pensione all'età legale; molto impopolare, è stato soppresso nel 2023."),
    ("AGIRC_ARRCO_T2",
        "Fino al 2018 lo status di quadro si leggeva direttamente sulla pensione: contributi AGIRC, garanzia \
        minima di punti, cassa dedicata. La fusione ha cancellato questi segni distintivi, e la definizione \
        stessa di quadro ha dovuto essere rinegoziata con un accordo interprofessionale nel 2020."),
    ("AGIRC_ARRCO_CEG_T1",
        "Nel 2019 la CEG ha raccolto l'eredità dell'AGFF, a sua volta creata nel 2001 per una ragione precisa: \
        quando nel 1982 arriva la pensione a 60 anni, i regimi complementari non seguono. Serve una \
        struttura dedicata (l'ASF nel 1983, poi l'AGFF) per finanziare il versamento delle pensioni \
        complementari senza riduzione tra 60 e 65 anni."),
    ("PREVOYANCE_CADRE_MIN",
        "Questo obbligo dell'1,50% risale all'articolo 7 del contratto collettivo dei quadri del 14 marzo \
        1947: un'assicurazione caso morte finanziata dal datore di lavoro, contropartita dell'attaccamento \
        dei quadri al proprio regime. Quando l'AGIRC è scomparsa nel 2019, l'obbligo ha rischiato di \
        scomparire con lei; l'accordo interprofessionale del 17 novembre 2017 lo ha mantenuto."),
    ("REDUCTION_FILLON",
        "Gli sgravi contributivi sui salari bassi iniziano con Édouard Balladur nel 1993, si estendono con lo \
        «sconto» Juppé, poi con gli aiuti legati alle 35 ore di Martine Aubry. La legge del 17 gennaio 2003, \
        voluta da François Fillon, ministro degli Affari sociali, li fonde in un unico dispositivo: da qui il \
        soprannome che gli è rimasto. Il rovescio è noto come «trappola dei bassi salari»: ogni euro di \
        aumento vicino allo SMIC (salario minimo) fa perdere al datore di lavoro parte dello sgravio. Il \
        rapporto Bozio-Wasmer (2024) lo ha quantificato e ha ispirato la riforma del 2026."),
    ("ALSACE_MOSELLE_MALADIE",
        "Quando l'Alsazia e la Mosella sono tedesche, dal 1871 al 1918, ricevono le assicurazioni sociali di \
        Bismarck: malattia (1883), infortuni (1884), vecchiaia (1889). Tornati alla Francia, gli abitanti \
        rifiutano di perdere diritti più avanzati di quelli della «Francia dell'interno»; la legge del \
        1° giugno 1924 mantiene il diritto locale. Il regime malattia complementare ne è l'erede diretto, \
        come i due giorni festivi in più (Venerdì santo e 26 dicembre)."),
    ("AIDE_POSTE_EA",
        "La legge del 10 luglio 1987 impone alle imprese con 20 o più dipendenti di impiegare almeno il 6% di \
        lavoratori con disabilità, pena un contributo. La legge dell'11 febbraio 2005, la grande legge sulla \
        disabilità, trasforma gli ex «laboratori protetti» in imprese adattate: imprese dell'ambiente \
        ordinario, soggette al diritto del lavoro, e non più strutture medico-sociali."),
    ("REDUC_SAL_HS",
        "«Lavorare di più per guadagnare di più»: lo slogan elettorale di Nicolas Sarkozy diventa la legge TEPA \
        dell'agosto 2007, che esenta gli straordinari da imposte e contributi. François Hollande sopprime la \
        maggior parte del dispositivo nel 2012; Emmanuel Macron lo ripristina, anticipato al 1° gennaio 2019 \
        sotto la pressione dei gilet gialli. Tre cambi di rotta in dodici anni per la stessa riga della busta \
        paga."),
    ("DFP_HS",
        "La deduzione forfettaria padronale è una sopravvivenza della legge TEPA del 2007: quando la riforma \
        del 2012 ha soppresso le esenzioni, è stata conservata per le imprese con meno di 20 dipendenti. La \
        legge finanziaria correttiva dell'estate 2022 l'ha estesa alle imprese da 20 a 249 dipendenti, con \
        un importo più basso."),
    ("FPT_CNRACL",
        "La CNRACL è stata creata nel 1945 ed è tuttora gestita dalla Caisse des dépôts, da Bordeaux. A lungo \
        in avanzo grazie a una demografia favorevole, ha dovuto per decenni riversare parte delle proprie \
        risorse ad altri regimi a titolo di «compensazione demografica». L'invecchiamento dei propri iscritti \
        l'ha fatta scivolare in deficit, da cui gli aumenti dell'aliquota a carico del datore di lavoro \
        sostenuti da enti locali e ospedali."),

    // ──────────────────────────────── Suisse ───────────────────────────────
    ("CH_AVS",
        "L'AVS figurava già tra le rivendicazioni dello sciopero generale del novembre 1918. Un articolo \
        costituzionale viene accettato nel 1925, ma la prima legge è respinta nel 1931. Bisogna attendere la \
        votazione del 6 luglio 1947: circa l'80% di sì, con un'affluenza record. Più di recente il popolo ha \
        innalzato l'età pensionabile delle donne a 65 anni (AVS 21, settembre 2022, di stretta misura) e poi \
        accettato nel marzo 2024 una 13ª rendita, la prima iniziativa di sinistra sulle assicurazioni sociali \
        a superare l'asticella."),
    ("CH_AI",
        "Iscritta nella Costituzione già nel 1925 insieme all'AVS, l'assicurazione invalidità entra in vigore \
        solo nel 1960. Di fronte all'impennata del numero di rendite, la 5ª revisione (2008) ha elevato a \
        principio che «l'integrazione prevale sulla rendita»: l'AI finanzia prima di tutto il ritorno al \
        lavoro."),
    ("CH_APG",
        "Le IPG sono nate nel 1940 per compensare la perdita di salario dei soldati mobilitati. È questo \
        regime che è servito da veicolo all'assicurazione maternità, respinta più volte dal popolo (1984, \
        1987, 1999) prima di essere accettata nel settembre 2004. Il congedo di paternità di due settimane ha \
        seguito la stessa strada, approvato in votazione nel settembre 2020."),
    ("CH_AC",
        "Fino alla metà degli anni Settanta l'assicurazione contro la disoccupazione era facoltativa in \
        Svizzera. Lo shock petrolifero e l'ondata di licenziamenti che seguì portarono a renderla \
        obbligatoria: articolo costituzionale del 1976, legge (LADI) del 1982."),
    ("CH_LPP",
        "Il sistema dei «tre pilastri» è stato iscritto nella Costituzione dalla votazione del dicembre 1972, \
        preferito a un'iniziativa del Partito del lavoro che voleva una pensione popolare unica e generosa. \
        Ci vollero poi più di dieci anni perché la LPP entrasse in vigore, nel 1985."),
    ("CH_AAP",
        "La legge sull'assicurazione malattia e infortuni è accettata in votazione nel febbraio 1912. Crea la \
        SUVA, istituto pubblico con sede a Lucerna, che dal 1918 assicura gli infortuni nell'industria e \
        nei mestieri a rischio."),
    ("CH_AANP",
        "Particolarità svizzera: l'infortunio avvenuto fuori dal lavoro, sugli sci o facendo bricolage, è \
        coperto dall'assicurazione del dipendente. Ereditata dalla legge del 1911 e dalla SUVA, questa \
        copertura è stata estesa a tutti i settori dalla legge sull'assicurazione contro gli infortuni del \
        1981."),
    ("CH_IS",
        "A Ginevra l'imposta alla fonte dei frontalieri francesi è al centro di un accordo del 29 gennaio \
        1973: il cantone tassa i salari sul posto e versa ai dipartimenti dell'Ain e dell'Alta Savoia una \
        compensazione calcolata sulla massa salariale dei frontalieri. Altri otto cantoni rientrano in un \
        accordo del 1983, in base al quale il frontaliere è tassato in Francia."),

    // ────────────────────────────── Luxembourg ─────────────────────────────
    ("LU_AM",
        "Il Lussemburgo adotta l'assicurazione malattia obbligatoria degli operai già nel 1901, sul modello di \
        Bismarck. Per più di un secolo operai e impiegati privati appartengono a casse distinte; lo «statuto \
        unico» del 1° gennaio 2009 li riunisce in un'unica Cassa nazionale della salute."),
    ("LU_ME",
        "La Mutualità dei datori di lavoro è nata con lo statuto unico del 2009: sopprimendo la distinzione \
        tra operai e impiegati, la riforma ha generalizzato il mantenimento del salario in caso di malattia. \
        Perché le piccole imprese non ne sostengano da sole il costo, i datori di lavoro lo mutualizzano."),
    ("LU_AP",
        "Come i vicini, il Lussemburgo ha costruito le sue pensioni sull'esempio tedesco fin dall'inizio del \
        XX secolo. Il paese ha a lungo alimentato una riserva considerevole, gestita da un fondo di \
        compensazione, grazie all'alta proporzione di lavoratori frontalieri che versano contributi senza \
        essere ancora pensionati."),

    // ────────────────────────────── Allemagne ──────────────────────────────
    ("DE_KRANKENVERSICHERUNG",
        "La legge del 1883 sull'assicurazione malattia degli operai è la prima delle assicurazioni sociali di \
        Bismarck. Il cancelliere la annuncia nel messaggio imperiale del 17 novembre 1881; il calcolo è \
        politico: dopo aver messo al bando le organizzazioni socialiste (legge del 1878), vuole staccare gli \
        operai dalla socialdemocrazia offrendo loro una protezione venuta dallo Stato."),
    ("DE_UNFALLVERSICHERUNG",
        "Secondo pilastro bismarckiano, l'assicurazione infortuni risale al 1884. Le Berufsgenossenschaften, \
        associazioni di datori di lavoro per settore che la gestiscono ancora oggi, sono quasi antiche quanto \
        il regime stesso."),
    ("DE_RENTENVERSICHERUNG",
        "L'assicurazione invalidità e vecchiaia del 1889 fissava l'età della pensione a 70 anni. La grande \
        riforma del 1957, sotto Adenauer, indicizza le pensioni ai salari. Nel 1986 il ministro del Lavoro \
        Norbert Blüm affigge manifesti con «Die Rente ist sicher» («la pensione è sicura»); in Germania la \
        frase è diventata l'esempio per eccellenza della promessa politica messa alla berlina."),
    ("DE_ARBEITSLOSENVERSICHERUNG",
        "L'assicurazione tedesca contro la disoccupazione nasce nel 1927, sotto la Repubblica di Weimar. Tre \
        anni dopo, un disaccordo sul suo finanziamento fa cadere l'ultima grande coalizione di Weimar, nel \
        marzo 1930: una lite su pochi decimi di punto di contributo, alle soglie della crisi che travolgerà \
        la Repubblica."),
    ("DE_PFLEGEVERSICHERUNG",
        "Quinto ramo della sicurezza sociale, l'assicurazione per la non autosufficienza è creata nel 1995 da \
        Norbert Blüm. Per compensare il costo per i datori di lavoro viene soppresso un giorno festivo: il \
        Buß- und Bettag (Giorno di penitenza e preghiera). Solo la Sassonia l'ha conservato, e i suoi \
        dipendenti pagano in cambio una quota più alta del contributo. La maggiorazione per chi non ha figli \
        deriva da una sentenza della Corte costituzionale del 2001."),
    ("DE_LOHNSTEUER",
        "La ritenuta dell'imposta sui salari da parte del datore di lavoro risale alla riforma finanziaria di \
        Matthias Erzberger del 1920, che accentra l'imposta sul reddito a livello del Reich. Erzberger, già \
        detestato dalla destra nazionalista per aver firmato l'armistizio del 1918, viene assassinato nel \
        1921."),
    ("DE_SOLI",
        "Introdotto nel 1991 per finanziare la riunificazione sotto il cancelliere Kohl, il Soli doveva essere \
        provvisorio. Soppresso nel 2021 per circa il 90% dei contribuenti, sopravvive per i redditi più \
        alti; nel marzo 2025 la Corte costituzionale ha stabilito che questo mantenimento resta conforme \
        alla Legge fondamentale."),
    ("DE_KIRCHENSTEUER",
        "L'imposta ecclesiastica è la contropartita storica delle secolarizzazioni del 1803, quando i beni \
        della Chiesa furono trasferiti ai principi. La Costituzione di Weimar (1919) la garantisce, e la \
        Legge fondamentale del 1949 ne ha ripreso l'articolo. Lo Stato la riscuote insieme all'imposta sul \
        salario; vi si sfugge lasciando ufficialmente la propria Chiesa, passo che centinaia di migliaia di \
        tedeschi compiono ogni anno."),

    // ──────────────────────────────── Autriche ─────────────────────────────
    ("AT_SV",
        "L'Austria-Ungheria adotta l'assicurazione infortuni (1887) e l'assicurazione malattia (1888) sulla \
        scia della Germania. Il diritto attuale poggia sull'ASVG, la legge generale sulle assicurazioni \
        sociali del 1955, l'anno stesso in cui il paese recupera la piena sovranità."),
    ("AT_LOHNSTEUER",
        "Particolarità austriaca: la 13ª e la 14ª mensilità, versate d'estate e a fine anno, beneficiano di \
        un'aliquota fiscale ridotta forfettaria. Questo vantaggio, radicato da tempo nei contratti collettivi, \
        è una delle conquiste più difese del paese."),

    // ───────────────────────────────── Italie ──────────────────────────────
    ("IT_IVS",
        "L'antenata dell'INPS è una cassa nazionale di previdenza creata nel 1898, ad adesione volontaria. La \
        riforma Dini del 1995 fa passare l'Italia a un calcolo «contributivo», basato sui contributi versati. \
        Nel dicembre 2011, in piena crisi dei debiti, la ministra Elsa Fornero annuncia l'innalzamento \
        dell'età e il blocco dell'indicizzazione delle pensioni e scoppia in lacrime in piena conferenza \
        stampa: l'immagine ha fatto il giro del paese."),
    ("IT_TFR",
        "Il TFR ha sostituito nel 1982 la vecchia indennità di anzianità. Questo salario differito, \
        accantonato dal datore di lavoro, è servito a lungo come finanziamento a basso costo per le imprese \
        italiane. Dal 2007 il dipendente che non dice nulla vede il suo TFR destinato a un fondo pensione \
        («silenzio-assenso»); molti hanno scelto di lasciarlo in azienda."),
    ("IT_IRPEF",
        "L'IRPEF nasce dalla grande riforma tributaria del 1973-1974. Alla sua creazione contava 32 scaglioni, \
        dal 10% al 72%. Le riforme successive ne hanno lasciati solo pochi; ridurre il numero degli \
        scaglioni, o addirittura introdurre una «flat tax», è diventato un segno distintivo politico della \
        destra italiana."),
    ("IT_ADD_REG",
        "L'addizionale regionale è creata nel 1997, insieme all'IRAP, dal ministro delle Finanze Vincenzo \
        Visco, nel quadro del «federalismo fiscale»: le regioni finanziano il proprio sistema sanitario con \
        un'imposta di cui fissano l'aliquota. Risultato: lo stesso stipendio non è tassato allo stesso modo a \
        Milano e a Napoli."),
    ("IT_INAIL",
        "La legge del 17 marzo 1898 rende obbligatoria l'assicurazione degli operai dell'industria contro gli \
        infortuni, lo stesso anno della legge francese. L'INAIL, istituto unico, è creato nel 1933."),
    ("IT_NASPI",
        "La NASpI è uno dei tasselli del Jobs Act di Matteo Renzi (2015), che ha anche allentato la tutela \
        contro i licenziamenti prevista dal famoso articolo 18 dello Statuto dei lavoratori del 1970, al \
        prezzo di una rottura duratura con la CGIL."),
    ("IT_MATERNITA",
        "La legge 1204 del 1971 sulla tutela delle lavoratrici madri è stata una delle grandi conquiste degli \
        anni di mobilitazione sociale seguiti all'«autunno caldo» del 1969. Il congedo di paternità \
        obbligatorio è comparso solo nel 2012, per un solo giorno."),
    ("IT_BONUS_CUNEO",
        "Il «cuneo fiscale», lo scarto tra quanto costa un dipendente e quanto incassa, è un'ossessione \
        italiana. Il «bonus 80 euro» di Matteo Renzi (2014) ha aperto una serie di misure che ogni governo ha \
        rinominato e prorogato: Draghi, poi Meloni, lo hanno allargato e ne hanno cambiato la forma."),
    ("IT_ESONERO",
        "L'esonero dai contributi a carico del lavoratore è stato introdotto dal governo Draghi nel 2022 contro \
        l'inflazione, poi ampliato dal governo Meloni. Rinnovato di anno in anno, dal 2025 è stato \
        trasformato in un beneficio fiscale."),
    ("IT_FONDO_GARANZIA",
        "Il fondo di garanzia del TFR è stato creato dalla stessa legge del 1982 che ha istituito il TFR: \
        senza di esso, un salario differito per anni poteva sparire con il fallimento del datore di lavoro."),

    // ──────────────────────────────── Espagne ──────────────────────────────
    ("ES_CC",
        "La prima legge sociale spagnola è la legge Dato del 1900 sugli infortuni sul lavoro. L'Istituto \
        nazionale di previdenza è fondato nel 1908; ma la Sicurezza sociale moderna nasce solo con la legge \
        di base del 1963, entrata in vigore nel 1967, sotto il franchismo. Nel 1995 il «Patto di Toledo» \
        impegna tutti i partiti a tenere le pensioni fuori dalla battaglia elettorale."),
    ("ES_MEI",
        "Il meccanismo di equità intergenerazionale è opera del ministro José Luis Escrivá (2021-2023). \
        Sostituisce il «fattore di sostenibilità» della riforma Rajoy del 2013, che avrebbe ridotto le \
        pensioni con l'allungarsi della vita e non è mai stato applicato."),
    ("ES_FOGASA",
        "Il FOGASA è creato nel 1976, durante la transizione democratica, per garantire ai dipendenti il \
        pagamento dei salari in caso di insolvenza del datore di lavoro."),
    ("ES_DESEMPLEO",
        "La riforma del lavoro del 2021, negoziata dalla ministra Yolanda Díaz con sindacati e imprese, ha \
        fatto del contratto a tempo indeterminato la regola in un paese a lungo campione europeo dei \
        contratti a termine. I contributi di disoccupazione più pesanti sui contratti a termine ne sono uno \
        strumento."),

    // ──────────────────────────────── Portugal ─────────────────────────────
    ("PT_SS",
        "Sotto l'Estado Novo di Salazar la previdenza è organizzata in casse corporative per professione \
        (legge del 1935). Dopo la Rivoluzione dei garofani del 25 aprile 1974 queste casse vengono unificate \
        in una sicurezza sociale universale, sancita dalla Costituzione del 1976."),
    ("PT_IRS",
        "L'IRS è entrata in vigore il 1° gennaio 1989 e ha sostituito un mosaico di imposte cedolari. Da \
        allora il Portogallo ha moltiplicato i regimi speciali, come quello dei «residenti non abituali» \
        (2009), che attirava pensionati e dirigenti stranieri prima di essere chiuso ai nuovi arrivati nel \
        2024."),
    ("PT_FCT",
        "Il Fondo di compensazione del lavoro e il suo fondo di garanzia sono stati creati nel 2013, durante \
        il programma di assistenza della «troika» (FMI, BCE, Commissione europea), in cambio della riduzione \
        delle indennità di licenziamento."),

    // ──────────────────────────────── Belgique ─────────────────────────────
    ("BE_ONSS_SAL",
        "La sicurezza sociale belga nasce da un «patto sociale» negoziato in segreto durante l'Occupazione tra \
        imprenditori e sindacalisti. Il decreto-legge del 28 dicembre 1944 crea l'ONSS, che da allora \
        riscuote tutti i contributi in un unico luogo."),
    ("BE_ONSS_PAT",
        "La concertazione sociale belga si fonda dal 1944 sull'idea che imprese e sindacati gestiscano insieme \
        la sicurezza sociale. L'indicizzazione automatica dei salari, rara in Europa, ne è un altro pilastro: \
        è regolarmente contestata dai datori di lavoro in nome della competitività."),
    ("BE_PP",
        "La riforma fiscale del 1962 istituisce l'imposta sulle persone fisiche e la ritenuta professionale \
        trattenuta dal datore di lavoro. Il Belgio figura da tempo tra i paesi OCSE in cui il lavoro è più \
        tassato: ogni governo annuncia un «tax shift» per correggere questo tratto."),
    ("BE_BONUS_EMPLOI",
        "Il bonus all'impiego, creato nel 2005, risponde a un problema preciso: per un salario basso, il \
        guadagno netto di riprendere un lavoro anziché percepire un sussidio era talvolta quasi nullo. Lo si \
        chiama «trappola dell'impiego»."),
    ("BE_RED_STRUCT",
        "La riduzione strutturale è nata nel 2004 dalla fusione di diversi sgravi dei contributi a carico del \
        datore di lavoro. È l'equivalente belga della riduzione Fillon francese, con la stessa logica \
        decrescente."),

    // ──────────────────────────────── Royaume-Uni ──────────────────────────
    ("UK_NI_SAL",
        "La National Insurance nasce dal National Insurance Act del 1911, voluto da David Lloyd George: egli \
        vende la riforma con uno slogan rimasto celebre, «ninepence for fourpence» (nove pence di prestazioni \
        per quattro di contributo). Il rapporto Beveridge del 1942 e la legge del 1946 del governo Attlee ne \
        fanno la base dello Stato sociale britannico."),
    ("UK_NI_PAT",
        "L'aumento della National Insurance a carico dei datori di lavoro al 15% da aprile 2025, annunciato \
        nel primo bilancio di Rachel Reeves, prima donna Cancelliere dello Scacchiere, è stata la misura più \
        contestata dalle imprese britanniche di quel bilancio."),
    ("UK_INCOME_TAX",
        "L'imposta sul reddito britannica fu inventata da William Pitt il Giovane nel 1799 per finanziare la \
        guerra contro la Francia rivoluzionaria. Abolita nel 1816, ripristinata nel 1842 da Robert Peel, \
        doveva essere sempre temporanea. La ritenuta alla fonte, il PAYE, è introdotta nel 1944."),

    // ──────────────────────────────── Irlande ──────────────────────────────
    ("IE_USC",
        "La Universal Social Charge è stata introdotta dal bilancio 2011, nel pieno della crisi bancaria \
        irlandese e del piano di salvataggio europeo, in sostituzione di due prelievi precedenti. Pensata \
        come misura d'urgenza, è rimasta."),
    ("IE_PRSI",
        "L'attuale PRSI risale al 1979. L'aumento progressivo delle sue aliquote, deciso a partire dal 2024, \
        finanzia le pensioni di fronte all'invecchiamento della popolazione, dopo l'abbandono, sotto la \
        pressione popolare, del progetto di portare l'età pensionabile a 67 anni."),

    // ──────────────────────────────── Pays-Bas ─────────────────────────────
    ("NL_LOONHEFFING",
        "La pensione di base AOW, i cui contributi sono inclusi nella loonheffing, è stata istituita nel 1957 \
        dal primo ministro Willem Drees. Generazioni di pensionati hanno detto «trekken van Drees» \
        («prendere da Drees») per parlare della propria pensione."),
    ("NL_ZVW",
        "La legge sull'assicurazione sanitaria del 2006, voluta dal ministro Hans Hoogervorst, ha soppresso la \
        distinzione tra casse pubbliche e assicurazione privata: tutti i residenti sottoscrivono \
        un'assicurazione di base presso assicuratori privati in concorrenza, un modello unico in Europa."),
    ("NL_AOF",
        "La vecchia legge sull'invalidità, la WAO del 1967, è stata vittima del proprio successo: all'inizio \
        degli anni Novanta ne beneficiava quasi un milione di olandesi, e il primo ministro Ruud Lubbers \
        parlava di un paese «malato». La WIA l'ha sostituita nel 2006 puntando sulla capacità lavorativa \
        residua."),

    // ─────────────────────────────── Scandinavie ───────────────────────────
    ("SE_SKATT",
        "Nel 1976 Astrid Lindgren, la creatrice di Pippi Calzelunghe, scopre che le regole fiscali la portano \
        a un'aliquota marginale superiore al 100%. Pubblica una fiaba satirica, «Pomperipossa nel paese di \
        Monismania». Il dibattito che ne scaturisce contribuisce lo stesso anno alla sconfitta dei \
        socialdemocratici, al potere da 44 anni."),
    ("SE_ARBETSGIVARAVGIFT",
        "La riforma delle pensioni del 1994-1999, votata da cinque partiti, ha creato un sistema a conti \
        nozionali imitato in diversi paesi. Ogni anno gli svedesi ricevono una busta arancione che riepiloga \
        i loro diritti: «orange kuvertet» è diventato un simbolo nazionale."),
    ("DK_AM",
        "L'arbejdsmarkedsbidrag (contributo al mercato del lavoro) è stato creato nel 1994 dalla riforma \
        fiscale del governo socialdemocratico di Poul Nyrup Rasmussen. La Danimarca finanzia gran parte \
        della protezione sociale con le imposte anziché con i contributi, il che spiega uno dei livelli di \
        prelievo sul reddito più alti al mondo."),
    ("DK_ATP",
        "L'ATP, pensione complementare obbligatoria, è stata istituita nel 1964. Il suo contributo \
        forfettario, e non proporzionale al salario, ne fa una curiosità tra i regimi pensionistici europei."),
    ("FI_TYEL",
        "Nel 1962 la Finlandia ha istituito la pensione dei dipendenti del settore privato, gestita da \
        compagnie di assicurazione private con mandato pubblico: un modello originale di gestione \
        decentrata di un regime obbligatorio."),

    // ──────────────────────────────── Pays baltes ──────────────────────────
    ("EE_TULUMAKS",
        "Nel 1994, sotto il giovane primo ministro Mart Laar, l'Estonia diventa uno dei primi paesi d'Europa \
        ad adottare un'imposta sul reddito ad aliquota unica. L'esempio ha ispirato tutta l'Europa centrale e \
        orientale negli anni Duemila."),
    ("EE_KOGUMISPENSION",
        "Obbligatorio per le giovani generazioni dal 2002, il secondo pilastro è diventato facoltativo nel \
        2021 su iniziativa del partito Isamaa. Decine di migliaia di estoni ne sono usciti per recuperare i \
        propri risparmi."),
    ("EE_SOTSIAALMAKS",
        "L'imposta sociale estone del 33% è pagata interamente dal datore di lavoro e finanzia sia la \
        pensione sia la sanità. Il paese ha scelto un prelievo unico e leggibile anziché una serie di \
        contributi."),
    ("LT_SODRA",
        "Nel 2019 la Lituania ha trasferito quasi tutti i contributi del datore di lavoro sul dipendente, \
        alzando nel contempo i salari lordi di quasi il 29% per compensare: il salario netto non cambiava, \
        ma la busta paga rendeva visibile il costo reale della protezione sociale."),
    ("LV_IIN",
        "Nel 2018 la Lettonia ha abbandonato l'aliquota unica a favore di una scala progressiva, in \
        controtendenza rispetto all'orientamento che aveva segnato la regione dagli anni Novanta."),

    // ──────────────────────────── Europe centrale ──────────────────────────
    ("PL_EMERYTALNE",
        "La riforma del 1999 ha creato fondi pensione privati obbligatori, gli OFE. Nel 2014 il governo di \
        Donald Tusk trasferisce all'assicurazione pubblica ZUS circa metà dei loro attivi, per alleggerire il \
        debito pubblico: una delle più spettacolari marce indietro in materia di pensioni a capitalizzazione \
        in Europa."),
    ("PL_ZDROWOTNE",
        "Il «Polski Ład» (Nuovo ordine polacco), riforma fiscale del 2022 del governo PiS, ha soppresso la \
        deducibilità del contributo sanitario dall'imposta. La sua caotica entrata in vigore ha costretto a \
        correggere d'urgenza le buste paga di gennaio 2022 in cui alcuni salari netti erano diminuiti."),
    ("CZ_DAN",
        "Fino al 2020 l'imposta ceca era calcolata su un «salario super-lordo», che aggiungeva al lordo i \
        contributi a carico del datore di lavoro. Questa curiosità, introdotta nel 2008, è stata soppressa \
        nel 2021."),
    ("SK_DAN",
        "Nel 2004 la Slovacchia di Ivan Mikloš adotta un'aliquota unica del 19% su redditi, società e IVA, \
        diventando la vetrina europea della «flat tax». Il governo Fico reintroduce uno scaglione al 25% nel \
        2013."),
    ("HU_SZJA",
        "L'Ungheria di Viktor Orbán ha introdotto nel 2011 un'imposta sul reddito ad aliquota unica del 16%, \
        scesa al 15% nel 2016. Per politica natalista, le madri di quattro figli ne sono esenti dal 2020; \
        gli under 25 lo sono dal 2022, entro un certo limite."),
    ("HU_SZOCHO",
        "Il contributo sociale ungherese a carico del datore di lavoro è stato abbassato passo dopo passo, dal \
        27% nel 2016 al 13% nel 2022, nel quadro di accordi salariali con le parti sociali: meno oneri in \
        cambio di un salario minimo più alto."),
    ("RO_CAS",
        "Nel 2018 la Romania ha trasferito quasi tutti i contributi sociali dal datore di lavoro al \
        dipendente, esigendo che i salari lordi fossero aumentati di conseguenza. La misura, soprannominata \
        «rivoluzione fiscale», spiega perché il dipendente rumeno sopporta la maggior parte dei contributi \
        sulla busta paga."),
    ("RO_IMPOZIT",
        "La Romania ha adottato un'imposta ad aliquota unica nel 2005 (16%), poi l'ha abbassata al 10% nel \
        2018, una delle aliquote più basse dell'Unione europea."),
    ("BG_DANAK",
        "Con la sua aliquota unica del 10%, istituita nel 2008, la Bulgaria applica una delle imposte sul \
        reddito più basse dell'Unione europea. Il 1° gennaio 2026 ha adottato l'euro, il che ha imposto di \
        convertire tutti i massimali contributivi."),
    ("HR_POREZ",
        "La riforma fiscale del 2024 ha soppresso il «prirez», sovrimposta comunale sull'imposta sul reddito, \
        e ha lasciato alle città il compito di fissare esse stesse le aliquote dell'imposta sul reddito entro \
        una forbice di legge."),
    ("GR_EFKA",
        "L'EFKA è stato creato nel 2017 per riunire una moltitudine di casse professionali, tra cui l'IKA dei \
        dipendenti privati. Durante la crisi dei debiti sovrani le pensioni greche sono state ridotte più \
        volte in applicazione dei memorandum firmati con i creditori."),
    ("CY_GESY",
        "Cipro ha avuto un sistema sanitario universale solo nel 2019, con il lancio del GESY, atteso da una \
        legge del 2001. Fino ad allora gran parte delle cure era pagata direttamente o tramite assicurazione \
        privata."),

    // ───────────────────────────── Micro-États ─────────────────────────────
    ("AD_IRPF",
        "Andorra non ha conosciuto alcuna imposta sul reddito delle persone fisiche fino al 2015. La sua \
        introduzione, con un'aliquota massima del 10%, fa parte degli impegni presi dal principato per uscire \
        dalle liste dei paradisi fiscali e negoziare con l'Unione europea."),
    ("MC_CAR",
        "Monaco non riscuote imposta sul reddito da quando il principe Carlo III l'ha abolita nel 1869, grazie \
        alle entrate del casinò. Solo i francesi non vi sfuggono: dopo la crisi del 1962, durante la quale il \
        generale de Gaulle fa installare controlli doganali alla frontiera, la convenzione fiscale del 1963 \
        li sottopone all'imposta francese."),

    // ──────────────────────────────── Amérique du Nord ─────────────────────
    ("US_SS",
        "Il Social Security Act è firmato da Franklin D. Roosevelt il 14 agosto 1935, in piena Grande \
        Depressione. La prima pensionata, Ida May Fuller, maestra del Vermont, aveva versato meno di 25 \
        dollari; è vissuta fino a 100 anni e ha ricevuto quasi 23.000 dollari di pensione."),
    ("US_MEDICARE",
        "Medicare è creato nel 1965 da Lyndon B. Johnson nel quadro della «Great Society». Firma la legge a \
        Independence (Missouri), alla presenza dell'ex presidente Harry Truman, il cui progetto di \
        assicurazione sanitaria era fallito: Truman riceve la prima tessera Medicare."),
    ("US_ADD_MEDICARE",
        "La sovrimposta dello 0,9% sui redditi alti è stata introdotta nel 2013 dall'Affordable Care Act, \
        l'«Obamacare», per finanziare la riforma dell'assicurazione sanitaria."),
    ("US_FUTA",
        "La disoccupazione federale fa parte del Social Security Act del 1935. Il suo meccanismo di credito, \
        che riduce un'aliquota del 6% allo 0,6% per i datori di lavoro che contribuiscono a un regime \
        statale, è stato concepito per spingere ogni Stato a creare la propria assicurazione contro la \
        disoccupazione."),
    ("US_IMPOT_FED",
        "L'imposta federale sul reddito è stata resa possibile solo dal 16° emendamento del 1913, dopo che la \
        Corte suprema l'aveva giudicata incostituzionale nel 1895. La ritenuta alla fonte è istituita nel \
        1943, per finanziare la guerra, su un'idea dell'economista Beardsley Ruml."),
    ("US_IMPOT_STATE",
        "La sovrimposta californiana dell'1% oltre un milione di dollari di reddito è stata creata dalla \
        Proposition 63, approvata per referendum nel 2004 per finanziare i servizi di salute mentale. La \
        California applica così l'aliquota marginale d'imposta statale più alta degli Stati Uniti."),
    ("US_CA_SDI",
        "Nel 2004 la California è stata il primo Stato americano a istituire un congedo familiare retribuito, \
        finanziato da questo contributo. Gli Stati Uniti restano l'unico paese ricco senza congedo di \
        maternità retribuito a livello federale."),
    ("CA_RPC2",
        "Il potenziamento del regime pensionistico canadese è stato concordato nel 2016 tra Ottawa e le \
        province, prima estensione importante dalla sua creazione; è attuato progressivamente dal 2019, e il \
        Québec ha potenziato il proprio regime con lo stesso calendario."),
    ("ON_IMPOT_PROV",
        "A differenza del Québec, l'Ontario affida la riscossione della propria imposta sul reddito all'Agenzia \
        delle entrate del Canada, in virtù di un accordo di riscossione fiscale: il dipendente dell'Ontario \
        compila una sola dichiarazione."),
    ("CA_RPC",
        "Il Canada Pension Plan è creato nel 1965 sotto il governo di Lester B. Pearson. Il Québec di Jean \
        Lesage rifiuta di aderirvi e crea un proprio regime; le riserve del Québec finanziano la Caisse de \
        dépôt et placement du Québec, diventata uno dei maggiori investitori istituzionali del paese."),
    ("CA_AE",
        "Una prima legge federale sull'assicurazione contro la disoccupazione, adottata nel 1935 dal governo \
        Bennett, è annullata dai tribunali in nome della ripartizione delle competenze. Bisogna modificare la \
        Costituzione nel 1940 perché l'assicurazione federale contro la disoccupazione veda la luce."),
    ("CA_IMPOT_FED",
        "L'imposta federale sul reddito è introdotta nel 1917 dall'Income War Tax Act (legge sull'imposta di \
        guerra sul reddito), presentata come misura temporanea per finanziare la Prima guerra mondiale."),
    ("QC_RRQ",
        "Il RRQ è frutto della «Rivoluzione tranquilla»: nel 1964-1965 il governo di Jean Lesage negozia con \
        Ottawa il diritto di avere un proprio regime. I contributi accumulati danno vita, nel 1965, alla \
        Caisse de dépôt et placement du Québec."),
    ("QC_RQAP",
        "Il Québec ha ottenuto di gestire le proprie prestazioni parentali con un'intesa con Ottawa nel 2005, \
        al termine di un lungo braccio di ferro; eppure lo stesso anno la Corte suprema riconosceva la \
        competenza federale su queste prestazioni. Il RQAP, lanciato nel 2006, ha creato settimane riservate \
        ai padri, che hanno fatto crescere fortemente il loro ricorso al congedo."),
    ("QC_IMPOT_PROV",
        "Il Québec è l'unica provincia a riscuotere da sé la propria imposta sul reddito. Maurice Duplessis \
        crea questa imposta provinciale nel 1954, per affermare l'autonomia fiscale del Québec di fronte a \
        Ottawa."),
    ("QC_FSS",
        "L'assicurazione malattia del Québec entra in vigore nel novembre 1970. Provoca uno sciopero dei medici \
        specialisti nell'ottobre 1970, in piena crisi d'Ottobre, che l'Assemblea nazionale fa cessare con una \
        legge speciale."),
    ("MX_IMSS",
        "L'Istituto messicano di sicurezza sociale è creato nel 1943 sotto il presidente Manuel Ávila Camacho. \
        Circa metà dei lavoratori messicani, occupati nell'economia informale, resta ancora oggi fuori dalla \
        sua copertura."),
    ("MX_INFONAVIT",
        "L'INFONAVIT, fondo per l'abitazione dei lavoratori, è creato nel 1972 sotto il presidente Luis \
        Echeverría. È diventato il primo erogatore di mutui ipotecari dell'America latina."),
    ("MX_RETIRO",
        "Il Messico ha creato nel 1992 un sistema di risparmio previdenziale individuale, poi nel 1997 le \
        Afores, gestori privati dei conti dei lavoratori, sul modello cileno."),

    // ────────────────────────────── Amérique du Sud ─────────────────────────
    ("BR_INSS",
        "La previdenza sociale brasiliana risale alla legge Eloy Chaves del 1923, che crea una cassa pensioni \
        per i ferrovieri. L'INSS, istituto unico, è creato nel 1990."),
    ("BR_FGTS",
        "Il FGTS è creato nel 1966, sotto il regime militare, per sostituire la stabilità del posto garantita \
        ai dipendenti dopo dieci anni di anzianità. I datori di lavoro guadagnavano la libertà di licenziare; \
        i dipendenti, un capitale utilizzabile per comprare casa."),

    // ───────────────────────────────── Asie ────────────────────────────────
    ("JP_KENPO",
        "La legge giapponese sull'assicurazione malattia dei dipendenti è adottata nel 1922 e applicata dal \
        1927. L'assicurazione malattia universale, estesa a tutta la popolazione, è realizzata nel 1961."),
    ("JP_KOSEI",
        "L'assicurazione pensionistica dei dipendenti nasce nel 1942, durante la guerra. Nel 2007 si scopre \
        che circa 50 milioni di posizioni contributive non possono essere attribuite a nessuno: lo scandalo \
        delle «pensioni scomparse» contribuisce alla sconfitta del primo governo di Shinzō Abe alle elezioni \
        per la Camera alta del 2007."),
    ("JP_ROUSAI",
        "L'assicurazione giapponese contro gli infortuni sul lavoro è creata nel 1947, lo stesso anno della \
        legge sulle norme del lavoro, nel quadro delle riforme del dopoguerra."),
    ("JP_KAIGO",
        "L'assicurazione per l'assistenza di lungo periodo, in vigore dal 2000, è stata una delle risposte \
        del paese all'invecchiamento più rapido del mondo. Vi si contribuisce a partire dai 40 anni."),
    ("JP_KOYO",
        "L'assicurazione per l'impiego del 1974 ha sostituito l'assicurazione contro la disoccupazione creata \
        nel 1947. Finanzia anche aiuti al mantenimento dell'occupazione, eredità di una cultura dell'impiego \
        a vita."),
    ("JP_SHOTOKUZEI",
        "Dal 2013 e fino al 2037 l'imposta sul reddito è maggiorata di una sovrimposta speciale per la \
        ricostruzione del 2,1%, che finanzia la ricostruzione dopo il terremoto e lo tsunami dell'11 marzo \
        2011."),
    ("JP_JUMINZEI",
        "L'imposta di residenza è calcolata sui redditi dell'anno precedente. I giovani laureati quindi non la \
        pagano durante il primo anno di lavoro e scoprono a giugno del secondo anno un calo del salario \
        netto."),
    ("CN_GONGJIJIN",
        "Il fondo per l'abitazione si ispira al Central Provident Fund di Singapore. Shanghai lo sperimenta nel \
        1991, nel quadro della riforma che pone fine all'alloggio assegnato dall'unità di lavoro."),
    ("CN_IIT",
        "L'imposta sul reddito cinese è istituita nel 1980 con una soglia di 800 yuan al mese, che in pratica \
        la riservava agli stranieri. La riforma del 2018 alza la soglia a 5.000 yuan e introduce deduzioni \
        per l'istruzione dei figli, l'alloggio o i genitori anziani."),
    ("CN_YANGLAO",
        "La riforma degli anni Novanta combina un fondo comune e conti individuali. Nel 2024 la Cina ha avviato \
        il primo innalzamento progressivo dell'età pensionabile dagli anni Cinquanta."),
    ("KR_NPS",
        "La pensione nazionale coreana nasce nel 1988. Di fronte all'esaurimento previsto delle riserve, nel \
        marzo 2025 il Parlamento ha votato una riforma che porta progressivamente l'aliquota contributiva dal \
        9% al 13%, primo aumento dal 1998."),
    ("KR_NHI",
        "La Corea del Sud ha realizzato la copertura sanitaria universale nel 1989, appena dodici anni dopo la \
        creazione dell'assicurazione malattia obbligatoria per le grandi imprese, nel 1977."),
    ("KR_EI",
        "L'assicurazione coreana per l'impiego è istituita nel 1995. La crisi finanziaria asiatica del \
        1997-1998, che fa esplodere la disoccupazione, porta a estenderla d'urgenza a tutte le imprese."),
    ("KR_SANJAE",
        "L'assicurazione contro gli infortuni sul lavoro, creata nel 1964, è la prima assicurazione sociale \
        della storia della Corea del Sud."),
    ("KR_LTC",
        "L'assicurazione per l'assistenza di lungo periodo è stata introdotta nel 2008. La Corea del Sud ha \
        conosciuto uno degli invecchiamenti più rapidi dell'OCSE, con il tasso di fecondità più basso del \
        mondo."),
    ("IN_EPF",
        "Il fondo di previdenza dei dipendenti è creato nel 1952. Copre solo il settore formale, mentre la \
        grande maggioranza dei lavoratori indiani appartiene all'economia informale."),
    ("IN_ESI",
        "L'Employees' State Insurance Act è adottato nel 1948, appena un anno dopo l'indipendenza: una delle \
        prime leggi sociali dell'India indipendente."),
    ("IN_IMPOT",
        "Il bilancio 2020 ha introdotto un «nuovo regime» d'imposta, con aliquote più basse ma senza la \
        maggior parte delle deduzioni. Diventato il regime predefinito nel 2023, lascia al dipendente la \
        scelta ogni anno."),
    ("AE_EXPAT",
        "Gli Emirati non conoscono imposta sul reddito. Hanno introdotto l'IVA nel 2018 e un'imposta sulle \
        società del 9% nel 2023, mantenendo l'assenza di imposte sui salari."),

    // ──────────────────────────────── Océanie ──────────────────────────────
    ("AU_SUPER",
        "La Superannuation Guarantee è introdotta nel 1992 sotto il primo ministro Paul Keating. La previdenza \
        obbligatoria a capitalizzazione ha fatto dei fondi pensione australiani uno dei maggiori serbatoi di \
        risparmio previdenziale del mondo."),
    ("AU_MEDICARE",
        "L'assicurazione malattia universale australiana è nata due volte: Medibank, creato nel 1975 dal \
        governo Whitlam, poi smantellato dal successore, e Medicare, ripristinato nel 1984 da Bob Hawke, \
        finanziato da questo contributo."),
    ("AU_INCOME_TAX",
        "Nel 1942, durante la guerra, il governo federale australiano toglie agli Stati il diritto di \
        riscuotere l'imposta sul reddito, «prendendola in prestito» per la durata del conflitto. Non l'ha \
        mai restituita."),
    ("NZ_ACC",
        "Il rapporto del giudice Owen Woodhouse (1967) sfocia nel 1974 in un sistema unico al mondo: ogni \
        vittima di un infortunio è indennizzata senza colpa, ma in cambio rinuncia al diritto di citare in \
        giudizio il responsabile."),
    ("NZ_KIWISAVER_EMP",
        "KiwiSaver è lanciato nel 2007 dal ministro delle Finanze laburista Michael Cullen. L'adesione è \
        automatica per i nuovi assunti, che possono uscirne: un esempio spesso citato di «spinta gentile» \
        (nudge) in economia comportamentale."),
];
