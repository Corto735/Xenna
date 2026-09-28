// Anecdotes in English — faithful translation of `fr.rs` (reference text).

pub const TEXTES: &[(&str, &str)] = &[
    // ─────────────────────────────── France ───────────────────────────────
    ("SS_MALADIE",
        "French Social Security was born of the ordinances of 4 and 19 October 1945, in the wake of the \
        programme of the National Council of the Resistance, “Les Jours heureux” (“Happy Days”, March 1944). \
        The senior civil servant Pierre Laroque drew up its architecture; the Communist Labour minister \
        Ambroise Croizat set up its funds at a forced pace in 1946. The plan for a single fund ran into the \
        opposition of private-practice doctors, the mutual societies, the schemes already in place and \
        managerial staff (cadres), who refused to be merged into it: hence the patchwork of schemes that \
        survives today. The real first milestone came earlier: the social insurance laws of 1928 and 1930."),
    ("SS_VIEILLESSE_PLAF",
        "The first law on workers' and peasants' pensions (5 April 1910) set the retirement age at 65, when \
        few workers lived that long: the CGT union fought it as “pensions for the dead”. Pay-as-you-go took \
        hold in 1941, under Vichy, with the allowance for elderly employees, and 1945 extended it to all. \
        Retirement at 60 came with the ordinance of 26 March 1982. Since then, every reform has moved a \
        dial: Balladur (1993, pension based on the best 25 years instead of 10), Fillon (2003), Woerth \
        (2010, age 62), Touraine (2014, contribution period), Borne (2023, age 64, forced through with \
        Article 49.3 after months of demonstrations)."),
    ("SS_VIEILLESSE_DEPLAF",
        "Pensions have remained the most explosive reform in the country. In November-December 1995, the \
        Juppé plan on the special pension schemes triggered three weeks of transport strikes, and the \
        government withdrew its pensions component."),
    ("FAMILLE",
        "Family allowances grew out of employers' initiatives: during the Great War, industrialists paid a \
        “family supplement” on top of wages, then organised themselves into equalisation funds so that no \
        employer would be penalised for hiring fathers. The Landry law of 11 March 1932 made membership \
        compulsory in industry and commerce; the Family Code of 1939 hardened its pro-natalist slant. \
        Universality, a principle of 1945, was broken in 2015 when allowances were scaled to income, \
        against the advice of family associations."),
    ("AT_MP",
        "Before 1898, an injured worker had to prove his employer's fault before a judge: in other words, \
        almost never. It took eighteen years of back-and-forth in Parliament, from the bill tabled in 1880 \
        by Martin Nadaud, a former mason from the Creuse turned MP, to wrest the compromise of the law of \
        9 April 1898: automatic compensation, but at a flat rate. The law of 25 October 1919 extended it to \
        the first occupational diseases, such as lead poisoning. The asbestos scandal — asbestos was banned \
        in France on 1 January 1997 — led in 2000 to the creation of a dedicated compensation fund, the FIVA."),
    ("CSG_DEDUCTIBLE",
        "The CSG is the work of Prime Minister Michel Rocard, in the 1991 Finance Act: 1.1% on almost all \
        income, including income from capital. The text went through under Article 49.3, and the censure \
        motion that followed fell just five votes short of a majority. Tax or contribution? The legal \
        debate lasted years; the European Court of Justice ended up treating it as a social security \
        contribution. Created as a modest levy, it now raises more than income tax."),
    ("CSG_NON_DEDUCTIBLE",
        "The 1.7-point increase of 2018, which funded the abolition of employee health and unemployment \
        contributions, benefited workers but hit pensioners with nothing in return. Their anger fed the \
        yellow vests movement: as early as December 2018, the government created an intermediate rate to \
        spare modest pensions."),
    ("CRDS",
        "Created by the ordinance of 24 January 1996 (Juppé plan) to fund the CADES, the body in charge of \
        paying off the social security debt, the CRDS was due to end in 2009. Each new wave of deficits, \
        most recently the Covid-related debt, has pushed the deadline back, now set at 2033: the \
        “temporary” levy has passed the thirty-year mark."),
    ("CHOMAGE",
        "Unédic was born of an agreement signed on 31 December 1958 between employers and unions, at the \
        urging of General de Gaulle, who preferred a jointly managed scheme to State management. André \
        Bergeron, leader of Force ouvrière, made it the symbol of joint management (paritarisme) for \
        decades. The balance has been reversed since 2018: with the employee contribution abolished, the \
        State now sets the framework for negotiations through a framing letter, and has taken back control \
        by decree when the social partners failed to agree."),
    ("AGIRC_ARRCO_T1",
        "In 1946-1947 managerial staff refused to be merged into the general scheme: the collective \
        agreement of 14 March 1947 gave them their own points-based fund, AGIRC. Non-managerial staff \
        obtained the equivalent through the agreement of 8 December 1961, which founded ARRCO. The two \
        merged on 1 January 2019 (agreement of 30 October 2015). The same agreement had created a 10% \
        “penalty” for three years for anyone retiring at the legal age; highly unpopular, it was \
        abolished in 2023."),
    ("AGIRC_ARRCO_T2",
        "Until 2018, managerial status could be read directly in one's pension: AGIRC contributions, a \
        guaranteed minimum of points, a dedicated fund. The merger erased these distinctive signs, and the \
        very definition of a cadre had to be renegotiated in a cross-industry agreement in 2020."),
    ("AGIRC_ARRCO_CEG_T1",
        "In 2019 the CEG took over from the AGFF, itself created in 2001 for a specific reason: when \
        retirement at 60 arrived in 1982, the supplementary schemes did not follow. A dedicated structure \
        was needed (the ASF in 1983, then the AGFF) to fund the payment of supplementary pensions without \
        reduction between 60 and 65."),
    ("PREVOYANCE_CADRE_MIN",
        "This 1.50% obligation dates back to Article 7 of the collective agreement for managerial staff of \
        14 March 1947: death insurance funded by the employer, in return for the cadres' attachment to \
        their own scheme. When AGIRC disappeared in 2019, the obligation nearly disappeared with it; the \
        cross-industry agreement of 17 November 2017 kept it."),
    ("REDUCTION_FILLON",
        "Contribution relief on low wages began with Édouard Balladur in 1993, expanded with the Juppé \
        “rebate”, then with Martine Aubry's aid linked to the 35-hour week. The law of 17 January 2003, \
        carried by François Fillon, Minister of Social Affairs, merged them into a single scheme: hence the \
        nickname that stuck. Its downside is known as the “low-wage trap”: every euro of pay rise near the \
        minimum wage (SMIC) costs the employer part of the relief. The Bozio-Wasmer report (2024) \
        quantified it and inspired the 2026 overhaul."),
    ("ALSACE_MOSELLE_MALADIE",
        "While Alsace and Moselle were German, from 1871 to 1918, they received Bismarck's social \
        insurance: sickness (1883), accidents (1884), old age (1889). Back in France, their inhabitants \
        refused to give up rights more advanced than those of “inner France”; the law of 1 June 1924 \
        maintained the local law. The supplementary health scheme is its direct heir, as are the two \
        extra public holidays (Good Friday and 26 December)."),
    ("AIDE_POSTE_EA",
        "The law of 10 July 1987 requires companies with 20 or more employees to employ at least 6% \
        disabled workers, failing which they pay a contribution. The law of 11 February 2005, the major \
        disability law, turned the former “sheltered workshops” into adapted enterprises: ordinary \
        businesses, subject to labour law, and no longer medico-social structures."),
    ("REDUC_SAL_HS",
        "“Work more to earn more”: Nicolas Sarkozy's campaign slogan became the TEPA law of August 2007, \
        which exempted overtime from tax and contributions. François Hollande abolished most of the scheme \
        in 2012; Emmanuel Macron restored it, brought forward to 1 January 2019 under pressure from the \
        yellow vests. Three changes of course in twelve years for the same payslip line."),
    ("DFP_HS",
        "The flat-rate employer deduction is a survival of the 2007 TEPA law: when the 2012 reform abolished \
        the exemptions, it was kept for companies with fewer than 20 employees. The amending finance law of \
        summer 2022 extended it to companies with 20 to 249 employees, at a lower amount."),
    ("FPT_CNRACL",
        "The CNRACL was created in 1945 and is still managed by the Caisse des dépôts, from Bordeaux. Long \
        in surplus thanks to favourable demographics, for decades it had to hand over part of its \
        resources to other schemes under “demographic compensation”. The ageing of its own members tipped \
        it into deficit, hence the increases in the employer rate borne by local authorities and \
        hospitals."),

    // ──────────────────────────────── Suisse ───────────────────────────────
    ("CH_AVS",
        "Old-age and survivors' insurance (AVS) was already among the demands of the general strike of \
        November 1918. A constitutional article was accepted in 1925, but the first law was rejected in \
        1931. It took the vote of 6 July 1947: around 80% yes, with record turnout. More recently, the \
        people raised the retirement age for women to 65 (AVS 21, September 2022, by a narrow majority), \
        then accepted a 13th pension payment in March 2024, the first left-wing initiative on social \
        insurance to pass."),
    ("CH_AI",
        "Written into the Constitution as early as 1925 alongside the AVS, disability insurance only came \
        into force in 1960. Faced with the surge in the number of pensions, the 5th revision (2008) made it \
        a principle that “rehabilitation comes before pension”: the AI first funds the return to work."),
    ("CH_APG",
        "Income compensation (APG) was born in 1940 to make up for the lost wages of mobilised soldiers. \
        This scheme served as the vehicle for maternity insurance, rejected several times by the people \
        (1984, 1987, 1999) before being accepted in September 2004. Two weeks of paternity leave followed \
        the same path, approved by popular vote in September 2020."),
    ("CH_AC",
        "Until the mid-1970s, unemployment insurance was optional in Switzerland. The oil shock and the \
        wave of layoffs that followed led to making it compulsory: constitutional article of 1976, law \
        (LACI) of 1982."),
    ("CH_LPP",
        "The “three pillars” system was written into the Constitution by the vote of December 1972, \
        preferred to an initiative of the Party of Labour that wanted a single, generous people's pension. \
        It then took more than ten years for the LPP law to come into force, in 1985."),
    ("CH_AAP",
        "The law on sickness and accident insurance was accepted by popular vote in February 1912. It \
        created SUVA, a public institution based in Lucerne, which since 1918 has insured accidents in \
        industry and high-risk trades."),
    ("CH_AANP",
        "A Swiss peculiarity: an accident outside work, skiing or doing DIY, is covered by the employee's \
        insurance. Inherited from the 1911 law and from SUVA, this cover was extended to all sectors by \
        the 1981 law on accident insurance."),
    ("CH_IS",
        "In Geneva, withholding tax on French cross-border workers is at the heart of an agreement of \
        29 January 1973: the canton taxes wages on the spot and pays the French departments of Ain and \
        Haute-Savoie a compensation calculated on the cross-border workers' payroll. Eight other cantons \
        fall under a 1983 agreement, under which the cross-border worker is taxed in France."),

    // ────────────────────────────── Luxembourg ─────────────────────────────
    ("LU_AM",
        "Luxembourg adopted compulsory health insurance for workers as early as 1901, on the Bismarck \
        model. For more than a century, blue-collar and private-sector white-collar workers belonged to \
        separate funds; the “single status” of 1 January 2009 brought them together in a single National \
        Health Fund."),
    ("LU_ME",
        "The Employers' Mutual was born with the 2009 single status: by abolishing the distinction between \
        blue- and white-collar workers, the reform generalised continued pay during sickness. So that \
        small businesses would not bear the cost alone, employers pool it."),
    ("LU_AP",
        "Like its neighbours, Luxembourg built its pensions on the German example from the early 20th \
        century. For a long time the country built up a considerable reserve, managed by a compensation \
        fund, thanks to the high proportion of cross-border workers who contribute without yet drawing a \
        pension."),

    // ────────────────────────────── Allemagne ──────────────────────────────
    ("DE_KRANKENVERSICHERUNG",
        "The 1883 law on workers' health insurance was the first of Bismarck's social insurances. The \
        chancellor announced it in the imperial message of 17 November 1881; the calculation was \
        political: having banned socialist organisations (1878 law), he wanted to detach workers from \
        Social Democracy by offering them protection from the State."),
    ("DE_UNFALLVERSICHERUNG",
        "The second Bismarckian pillar, accident insurance, dates from 1884. The Berufsgenossenschaften, \
        employers' associations by sector that still run it, are almost as old as the scheme itself."),
    ("DE_RENTENVERSICHERUNG",
        "The 1889 disability and old-age insurance set the pension age at 70. The major 1957 reform, under \
        Adenauer, indexed pensions to wages. In 1986, Labour Minister Norbert Blüm pasted up posters \
        saying “Die Rente ist sicher” (“pensions are safe”); in Germany the phrase has become the very \
        example of a political promise met with irony."),
    ("DE_ARBEITSLOSENVERSICHERUNG",
        "German unemployment insurance was born in 1927, under the Weimar Republic. Three years later, a \
        disagreement over its funding brought down Weimar's last grand coalition, in March 1930: a quarrel \
        over a few tenths of a contribution point, on the threshold of the crisis that would sweep the \
        Republic away."),
    ("DE_PFLEGEVERSICHERUNG",
        "The fifth branch of social security, long-term care insurance was created in 1995 by Norbert \
        Blüm. To offset the cost for employers, a public holiday was abolished: Buß- und Bettag (Day of \
        Repentance and Prayer). Only Saxony kept it, and in exchange its employees pay a higher share of \
        the contribution. The surcharge for childless people stems from a 2001 ruling of the \
        Constitutional Court."),
    ("DE_LOHNSTEUER",
        "Withholding of wage tax by the employer dates from Matthias Erzberger's financial reform of 1920, \
        which centralised income tax at Reich level. Erzberger, already hated by the nationalist right for \
        having signed the 1918 armistice, was assassinated in 1921."),
    ("DE_SOLI",
        "Introduced in 1991 to fund reunification under Chancellor Kohl, the Soli was meant to be \
        temporary. Abolished in 2021 for about 90% of taxpayers, it remains for the highest incomes; in \
        March 2025 the Constitutional Court ruled that keeping it was still compatible with the Basic Law."),
    ("DE_KIRCHENSTEUER",
        "Church tax is the historical counterpart of the secularisations of 1803, when Church property was \
        transferred to the princes. The Weimar Constitution (1919) guaranteed it, and the Basic Law of 1949 \
        took over the article. The State collects it together with wage tax; one escapes it by officially \
        leaving one's Church, a step hundreds of thousands of Germans take every year."),

    // ──────────────────────────────── Autriche ─────────────────────────────
    ("AT_SV",
        "Austria-Hungary adopted accident insurance (1887) and health insurance (1888) in Germany's wake. \
        Current law rests on the ASVG, the General Social Insurance Act of 1955, the very year the country \
        regained its full sovereignty."),
    ("AT_LOHNSTEUER",
        "An Austrian peculiarity: the 13th and 14th month's salaries, paid in summer and at the end of the \
        year, enjoy a reduced flat tax rate. This advantage, long anchored in collective agreements, is \
        one of the country's most fiercely defended entitlements."),

    // ───────────────────────────────── Italie ──────────────────────────────
    ("IT_IVS",
        "The ancestor of INPS was a national provident fund created in 1898, with voluntary membership. The \
        Dini reform of 1995 switched Italy to a “contributory” calculation, based on the contributions \
        paid. In December 2011, at the height of the debt crisis, minister Elsa Fornero announced the \
        raising of the retirement age and the freezing of pension indexation, and burst into tears in the \
        middle of the press conference: the image went round the country."),
    ("IT_TFR",
        "The TFR replaced the former seniority allowance in 1982. This deferred salary, set aside by the \
        employer, long served as cheap financing for Italian companies. Since 2007, an employee who says \
        nothing sees their TFR directed to a pension fund (“silence means consent”); many have chosen to \
        keep it in the company."),
    ("IT_IRPEF",
        "IRPEF was born of the major tax reform of 1973-1974. When it was created, it had 32 brackets, from \
        10% to 72%. Successive reforms have left only a few; cutting the number of brackets, or even \
        introducing a “flat tax”, has become a political marker of the Italian right."),
    ("IT_ADD_REG",
        "The regional surtax was created in 1997, together with IRAP, by Finance Minister Vincenzo Visco, \
        as part of “fiscal federalism”: the regions fund their health systems through a tax whose rate \
        they set. As a result, the same salary is not taxed the same way in Milan and in Naples."),
    ("IT_INAIL",
        "The law of 17 March 1898 made insurance of industrial workers against accidents compulsory, the \
        same year as the French law. INAIL, a single institute, was created in 1933."),
    ("IT_NASPI",
        "NASpI is one of the components of Matteo Renzi's Jobs Act (2015), which also relaxed the \
        protection against dismissal provided by the famous Article 18 of the 1970 Workers' Statute, at \
        the cost of a lasting rift with the CGIL union."),
    ("IT_MATERNITA",
        "Law 1204 of 1971 on the protection of working mothers was one of the great conquests of the years \
        of social mobilisation that followed the “Hot Autumn” of 1969. Compulsory paternity leave only \
        appeared in 2012, for a single day."),
    ("IT_BONUS_CUNEO",
        "The “cuneo fiscale” (tax wedge), the gap between what an employee costs and what they take home, is \
        an Italian obsession. Matteo Renzi's “80 euro bonus” (2014) opened a series of schemes that each \
        government has renamed and extended: Draghi, then Meloni, widened it and changed its form."),
    ("IT_ESONERO",
        "The exemption from employee contributions was created by the Draghi government in 2022 in response \
        to inflation, then expanded by the Meloni government. Renewed year after year, it was turned into \
        a tax benefit from 2025."),
    ("IT_FONDO_GARANZIA",
        "The TFR guarantee fund was created by the same 1982 law as the TFR itself: without it, a salary \
        deferred for years could vanish with the employer's bankruptcy."),

    // ──────────────────────────────── Espagne ──────────────────────────────
    ("ES_CC",
        "Spain's first social law was the Dato law of 1900 on workplace accidents. The National Provident \
        Institute was founded in 1908; but modern Social Security was only born with the Basic Law of \
        1963, in force from 1967, under Franco. In 1995, the “Toledo Pact” committed all parties to keeping \
        pensions out of electoral battles."),
    ("ES_MEI",
        "The intergenerational equity mechanism is the work of minister José Luis Escrivá (2021-2023). It \
        replaces the “sustainability factor” of Rajoy's 2013 reform, which would have reduced pensions as \
        life expectancy rose and was never applied."),
    ("ES_FOGASA",
        "FOGASA was created in 1976, during the democratic transition, to guarantee employees the payment of \
        their wages if the employer becomes insolvent."),
    ("ES_DESEMPLEO",
        "The 2021 labour reform, negotiated by minister Yolanda Díaz with unions and employers, made the \
        permanent contract the rule in a country long the European champion of temporary contracts. \
        Heavier unemployment contributions on temporary contracts are one of its instruments."),

    // ──────────────────────────────── Portugal ─────────────────────────────
    ("PT_SS",
        "Under Salazar's Estado Novo, welfare was organised in corporatist funds by profession (1935 law). \
        After the Carnation Revolution of 25 April 1974, these funds were unified into a universal social \
        security system, enshrined in the 1976 Constitution."),
    ("PT_IRS",
        "IRS came into force on 1 January 1989 and replaced a patchwork of schedular taxes. Portugal has \
        since multiplied special regimes, such as the “non-habitual residents” scheme (2009), which \
        attracted foreign retirees and executives before being closed to newcomers in 2024."),
    ("PT_FCT",
        "The Labour Compensation Fund and its guarantee fund were created in 2013, under the assistance \
        programme of the “troika” (IMF, ECB, European Commission), in exchange for lower severance pay."),

    // ──────────────────────────────── Belgique ─────────────────────────────
    ("BE_ONSS_SAL",
        "Belgian social security was born of a “social pact” negotiated in secret during the Occupation \
        between employers and trade unionists. The decree-law of 28 December 1944 created the ONSS, which \
        has since collected all contributions in a single place."),
    ("BE_ONSS_PAT",
        "Since 1944, Belgian social dialogue has rested on the idea that employers and unions jointly run \
        social security. Automatic wage indexation, rare in Europe, is another pillar: employers regularly \
        challenge it in the name of competitiveness."),
    ("BE_PP",
        "The 1962 tax reform established personal income tax and the professional withholding tax deducted \
        by the employer. Belgium has long been among the OECD countries where labour is most heavily taxed: \
        every government announces a “tax shift” to correct this."),
    ("BE_BONUS_EMPLOI",
        "The employment bonus, created in 2005, addresses a specific problem: for a low wage, the net gain \
        from taking a job rather than receiving a benefit was sometimes almost nil. This is known as the \
        “employment trap”."),
    ("BE_RED_STRUCT",
        "The structural reduction was born in 2004 from the merger of several employer contribution \
        reliefs. It is the Belgian equivalent of France's Fillon reduction, with the same tapering logic."),

    // ──────────────────────────────── Royaume-Uni ──────────────────────────
    ("UK_NI_SAL",
        "National Insurance was born of the National Insurance Act 1911, championed by David Lloyd George: \
        he sold the reform with a famous slogan, “ninepence for fourpence” (nine pence of benefits for \
        four of contributions). The Beveridge Report of 1942 and the Attlee government's 1946 Act made it \
        the foundation of the British welfare state."),
    ("UK_NI_PAT",
        "The rise in employer National Insurance to 15% from April 2025, announced in the first budget of \
        Rachel Reeves, the first woman Chancellor of the Exchequer, was the measure in that budget most \
        contested by British businesses."),
    ("UK_INCOME_TAX",
        "British income tax was invented by William Pitt the Younger in 1799 to fund the war against \
        revolutionary France. Abolished in 1816, reinstated in 1842 by Robert Peel, it was always meant to \
        be temporary. Withholding at source, PAYE, was introduced in 1944."),

    // ──────────────────────────────── Irlande ──────────────────────────────
    ("IE_USC",
        "The Universal Social Charge was introduced in the 2011 budget, at the height of the Irish banking \
        crisis and the European bailout, replacing two earlier levies. Conceived as an emergency measure, \
        it stayed."),
    ("IE_PRSI",
        "Today's PRSI dates from 1979. The gradual increase in its rates, decided from 2024, funds pensions \
        in the face of population ageing, after the plan to raise the pension age to 67 was dropped under \
        public pressure."),

    // ──────────────────────────────── Pays-Bas ─────────────────────────────
    ("NL_LOONHEFFING",
        "The AOW basic pension, whose contributions are included in the loonheffing, was introduced in 1957 \
        by Prime Minister Willem Drees. Generations of pensioners said “trekken van Drees” (“drawing from \
        Drees”) when talking about their pension."),
    ("NL_ZVW",
        "The 2006 Health Insurance Act, championed by minister Hans Hoogervorst, abolished the distinction \
        between public funds and private insurance: all residents take out basic insurance with competing \
        private insurers, a model unique in Europe."),
    ("NL_AOF",
        "The old disability law, the WAO of 1967, was a victim of its own success: in the early 1990s nearly \
        a million Dutch people were receiving it, and Prime Minister Ruud Lubbers spoke of a “sick” \
        country. The WIA replaced it in 2006, focusing on remaining capacity for work."),

    // ─────────────────────────────── Scandinavie ───────────────────────────
    ("SE_SKATT",
        "In 1976, Astrid Lindgren, the creator of Pippi Longstocking, discovered that the tax rules gave her \
        a marginal rate above 100%. She published a satirical tale, “Pomperipossa in Monismania”. The \
        debate it sparked contributed that same year to the defeat of the Social Democrats, in power for \
        44 years."),
    ("SE_ARBETSGIVARAVGIFT",
        "The 1994-1999 pension reform, voted by five parties, created a notional-account system imitated in \
        several countries. Every year, Swedes receive an orange envelope summarising their entitlements: \
        “orange kuvertet” has become a national symbol."),
    ("DK_AM",
        "The arbejdsmarkedsbidrag (labour market contribution) was created in 1994 by the tax reform of \
        Poul Nyrup Rasmussen's Social Democratic government. Denmark funds most of its social protection \
        through taxation rather than contributions, which explains one of the highest rates of tax on \
        income in the world."),
    ("DK_ATP",
        "ATP, a compulsory supplementary pension, was introduced in 1964. Its flat-rate contribution, not \
        proportional to salary, makes it a curiosity among European pension schemes."),
    ("FI_TYEL",
        "In 1962 Finland set up the pension scheme for private-sector employees, run by private insurance \
        companies under a public mandate: an original model of decentralised management of a compulsory \
        scheme."),

    // ──────────────────────────────── Pays baltes ──────────────────────────
    ("EE_TULUMAKS",
        "In 1994, under the young Prime Minister Mart Laar, Estonia became one of the first countries in \
        Europe to adopt a flat-rate income tax. The example inspired the whole of Central and Eastern Europe \
        in the 2000s."),
    ("EE_KOGUMISPENSION",
        "Compulsory for younger generations since 2002, the second pillar became optional in 2021 at the \
        initiative of the Isamaa party. Tens of thousands of Estonians left it to recover their savings."),
    ("EE_SOTSIAALMAKS",
        "Estonia's 33% social tax is paid entirely by the employer and funds both pensions and health. The \
        country chose a single, readable levy rather than a series of contributions."),
    ("LT_SODRA",
        "In 2019, Lithuania shifted almost all employer contributions onto the employee, while raising gross \
        wages by nearly 29% to compensate: net pay did not change, but the payslip made the real cost of \
        social protection visible."),
    ("LV_IIN",
        "In 2018 Latvia abandoned its flat-rate tax in favour of a progressive scale, going against the \
        trend that had marked the region since the 1990s."),

    // ──────────────────────────── Europe centrale ──────────────────────────
    ("PL_EMERYTALNE",
        "The 1999 reform created compulsory private pension funds, the OFEs. In 2014, Donald Tusk's \
        government transferred about half of their assets to the public insurer ZUS, to reduce public \
        debt: one of the most spectacular U-turns on funded pensions in Europe."),
    ("PL_ZDROWOTNE",
        "The “Polski Ład” (Polish Deal), the PiS government's 2022 tax reform, abolished the deductibility \
        of the health contribution from tax. Its chaotic entry into force forced emergency corrections to \
        January 2022 payslips on which some net salaries had fallen."),
    ("CZ_DAN",
        "Until 2020, Czech tax was calculated on a “super-gross salary”, which added employer contributions \
        to gross pay. This curiosity, introduced in 2008, was abolished in 2021."),
    ("SK_DAN",
        "In 2004, Ivan Mikloš's Slovakia adopted a single 19% rate on personal income, companies and VAT, \
        becoming Europe's showcase for the “flat tax”. The Fico government reintroduced a 25% bracket in \
        2013."),
    ("HU_SZJA",
        "Viktor Orbán's Hungary introduced a flat 16% income tax in 2011, lowered to 15% in 2016. In line \
        with its pro-natalist policy, mothers of four children have been exempt since 2020; under-25s have \
        been exempt since 2022, up to a certain limit."),
    ("HU_SZOCHO",
        "Hungary's employer social contribution was lowered step by step, from 27% in 2016 to 13% in 2022, \
        under wage agreements with the social partners: lower charges in exchange for a higher minimum \
        wage."),
    ("RO_CAS",
        "In 2018 Romania shifted almost all social contributions from the employer to the employee, \
        requiring gross wages to be raised accordingly. The measure, nicknamed the “fiscal revolution”, \
        explains why the Romanian employee bears most of the contributions on the payslip."),
    ("RO_IMPOZIT",
        "Romania adopted a flat-rate tax in 2005 (16%), then lowered it to 10% in 2018, one of the lowest \
        rates in the European Union."),
    ("BG_DANAK",
        "With its flat 10% tax, introduced in 2008, Bulgaria applies one of the lowest income taxes in the \
        European Union. On 1 January 2026 it adopted the euro, which required all contribution ceilings to \
        be converted."),
    ("HR_POREZ",
        "The 2024 tax reform abolished the “prirez”, a municipal surtax on income tax, and left it to cities \
        to set their own income tax rates within a legal range."),
    ("GR_EFKA",
        "EFKA was created in 2017 to bring together a multitude of occupational funds, including IKA for \
        private-sector employees. During the sovereign debt crisis, Greek pensions were cut many times \
        under the memoranda signed with the creditors."),
    ("CY_GESY",
        "Cyprus only got a universal health system in 2019, with the launch of GESY, awaited since a 2001 \
        law. Until then, much of healthcare was paid out of pocket or through private insurance."),

    // ───────────────────────────── Micro-États ─────────────────────────────
    ("AD_IRPF",
        "Andorra had no personal income tax until 2015. Its introduction, with a top rate of 10%, was part \
        of the commitments made by the principality to come off tax haven lists and negotiate with the \
        European Union."),
    ("MC_CAR",
        "Monaco has levied no income tax since Prince Charles III abolished it in 1869, thanks to casino \
        revenue. Only the French do not escape it: after the 1962 crisis, during which General de Gaulle \
        had customs checks set up at the border, the 1963 tax treaty made them subject to French tax."),

    // ──────────────────────────────── Amérique du Nord ─────────────────────
    ("US_SS",
        "The Social Security Act was signed by Franklin D. Roosevelt on 14 August 1935, in the midst of the \
        Great Depression. The first pensioner, Ida May Fuller, a Vermont schoolteacher, paid in less than \
        25 dollars; she lived to 100 and received nearly 23,000 dollars in benefits."),
    ("US_MEDICARE",
        "Medicare was created in 1965 by Lyndon B. Johnson as part of the “Great Society”. He signed the law \
        in Independence (Missouri), in the presence of former President Harry Truman, whose health \
        insurance plan had failed: Truman received the first Medicare card."),
    ("US_ADD_MEDICARE",
        "The 0.9% surtax on high incomes was introduced in 2013 by the Affordable Care Act, “Obamacare”, to \
        fund health insurance reform."),
    ("US_FUTA",
        "Federal unemployment is part of the 1935 Social Security Act. Its credit mechanism, which brings a \
        6% rate down to 0.6% for employers contributing to a state scheme, was designed to push every \
        state to create its own unemployment insurance."),
    ("US_IMPOT_FED",
        "Federal income tax was only made possible by the 16th Amendment of 1913, after the Supreme Court \
        had ruled it unconstitutional in 1895. Withholding at source was introduced in 1943, to fund the \
        war, on an idea of the economist Beardsley Ruml."),
    ("US_IMPOT_STATE",
        "California's 1% surtax on income above one million dollars was created by Proposition 63, adopted \
        by referendum in 2004 to fund mental health services. California thus has the highest marginal \
        state income tax rate in the United States."),
    ("US_CA_SDI",
        "In 2004 California became the first American state to introduce paid family leave, funded by this \
        contribution. The United States remains the only rich country without paid maternity leave at \
        federal level."),
    ("CA_RPC2",
        "The enhancement of the Canada Pension Plan was agreed in 2016 between Ottawa and the provinces, the \
        first major extension since its creation; it is being phased in from 2019, and Quebec enhanced its \
        own plan on the same timetable."),
    ("ON_IMPOT_PROV",
        "Unlike Quebec, Ontario entrusts the collection of its income tax to the Canada Revenue Agency, \
        under a tax collection agreement: an Ontario employee files a single return."),
    ("CA_RPC",
        "The Canada Pension Plan was created in 1965 under the government of Lester B. Pearson. Jean \
        Lesage's Quebec refused to join and created its own plan; Quebec's reserves fund the Caisse de \
        dépôt et placement du Québec, which has become one of the country's largest institutional \
        investors."),
    ("CA_AE",
        "A first federal unemployment insurance law, passed in 1935 by the Bennett government, was struck \
        down by the courts on the grounds of the division of powers. The Constitution had to be amended in \
        1940 for federal unemployment insurance to see the light of day."),
    ("CA_IMPOT_FED",
        "Federal income tax was introduced in 1917 by the Income War Tax Act, presented as a temporary \
        measure to fund the First World War."),
    ("QC_RRQ",
        "The Quebec Pension Plan (RRQ) is a product of the “Quiet Revolution”: in 1964-1965, Jean Lesage's \
        government negotiated with Ottawa the right to have its own plan. The accumulated contributions \
        gave birth, in 1965, to the Caisse de dépôt et placement du Québec."),
    ("QC_RQAP",
        "Quebec won the right to manage its own parental benefits through an agreement with Ottawa in 2005, \
        after a long tug-of-war; yet the same year the Supreme Court recognised federal jurisdiction over \
        these benefits. The RQAP, launched in 2006, created weeks reserved for fathers, who sharply \
        increased their use of leave."),
    ("QC_IMPOT_PROV",
        "Quebec is the only province that collects its own income tax. Maurice Duplessis created this \
        provincial tax in 1954, to assert Quebec's fiscal autonomy vis-à-vis Ottawa."),
    ("QC_FSS",
        "Quebec health insurance came into force in November 1970. It provoked a strike by medical \
        specialists in October 1970, in the midst of the October Crisis, which the National Assembly ended \
        with a special law."),
    ("MX_IMSS",
        "The Mexican Social Security Institute was created in 1943 under President Manuel Ávila Camacho. \
        About half of Mexican workers, employed in the informal economy, remain outside its coverage \
        today."),
    ("MX_INFONAVIT",
        "INFONAVIT, the workers' housing fund, was created in 1972 under President Luis Echeverría. It has \
        become the largest mortgage lender in Latin America."),
    ("MX_RETIRO",
        "In 1992 Mexico created an individual retirement savings system, then in 1997 the Afores, private \
        managers of employees' accounts, on the Chilean model."),

    // ────────────────────────────── Amérique du Sud ─────────────────────────
    ("BR_INSS",
        "Brazilian social insurance dates from the Eloy Chaves law of 1923, which created a pension fund for \
        railway workers. INSS, a single institute, was created in 1990."),
    ("BR_FGTS",
        "The FGTS was created in 1966, under the military regime, to replace the job security guaranteed to \
        employees after ten years of service. Employers gained the freedom to dismiss; employees, a capital \
        sum they could draw on to buy a home."),

    // ───────────────────────────────── Asie ────────────────────────────────
    ("JP_KENPO",
        "Japan's employees' health insurance law was passed in 1922 and applied from 1927. Universal health \
        insurance, extended to the whole population, was achieved in 1961."),
    ("JP_KOSEI",
        "Employees' pension insurance was born in 1942, during the war. In 2007 it emerged that about \
        50 million contribution records could not be matched to anyone: the “missing pensions” scandal \
        contributed to the defeat of Shinzō Abe's first government in the 2007 upper house elections."),
    ("JP_ROUSAI",
        "Japanese workers' accident insurance was created in 1947, the same year as the Labour Standards \
        Act, as part of the post-war reforms."),
    ("JP_KAIGO",
        "Long-term care insurance, in force since 2000, was one of the country's responses to the fastest \
        ageing in the world. Contributions start at age 40."),
    ("JP_KOYO",
        "The 1974 employment insurance replaced the unemployment insurance created in 1947. It also funds \
        job retention aid, a legacy of a culture of lifetime employment."),
    ("JP_SHOTOKUZEI",
        "From 2013 until 2037, income tax carries a special reconstruction surtax of 2.1%, which funds \
        reconstruction after the earthquake and tsunami of 11 March 2011."),
    ("JP_JUMINZEI",
        "Resident tax is calculated on the previous year's income. Young graduates therefore do not pay it \
        in their first year of work, and discover a drop in their net pay in June of their second year."),
    ("CN_GONGJIJIN",
        "The housing fund is inspired by Singapore's Central Provident Fund. Shanghai piloted it in 1991, as \
        part of the reform that ended housing allocated by the work unit."),
    ("CN_IIT",
        "Chinese income tax was introduced in 1980 with a threshold of 800 yuan a month, which in practice \
        reserved it for foreigners. The 2018 reform raised the threshold to 5,000 yuan and introduced \
        deductions for children's education, housing or elderly parents."),
    ("CN_YANGLAO",
        "The reform of the 1990s combined a pooled fund with individual accounts. In 2024, China began the \
        first gradual increase in its retirement age since the 1950s."),
    ("KR_NPS",
        "Korea's National Pension was born in 1988. Faced with the projected exhaustion of its reserves, \
        Parliament passed a reform in March 2025 that gradually raises the contribution rate from 9% to \
        13%, the first increase since 1998."),
    ("KR_NHI",
        "South Korea achieved universal health coverage in 1989, just twelve years after compulsory health \
        insurance was created for large companies in 1977."),
    ("KR_EI",
        "Korean employment insurance was introduced in 1995. The Asian financial crisis of 1997-1998, which \
        sent unemployment soaring, led to its emergency extension to all companies."),
    ("KR_SANJAE",
        "Workers' accident insurance, created in 1964, is the first social insurance in South Korea's \
        history."),
    ("KR_LTC",
        "Long-term care insurance was introduced in 2008. South Korea has experienced one of the fastest \
        rates of ageing in the OECD, with the lowest fertility rate in the world."),
    ("IN_EPF",
        "The Employees' Provident Fund was created in 1952. It covers only the formal sector, whereas the \
        vast majority of Indian workers belong to the informal economy."),
    ("IN_ESI",
        "The Employees' State Insurance Act was passed in 1948, just one year after independence: one of the \
        first social laws of independent India."),
    ("IN_IMPOT",
        "The 2020 budget introduced a “new regime” of income tax, with lower rates but without most \
        deductions. Made the default regime in 2023, it leaves employees the choice every year."),
    ("AE_EXPAT",
        "The Emirates have no income tax. They introduced VAT in 2018 and a 9% corporate tax in 2023, while \
        keeping salaries untaxed."),

    // ──────────────────────────────── Océanie ──────────────────────────────
    ("AU_SUPER",
        "The Superannuation Guarantee was introduced in 1992 under Prime Minister Paul Keating. Compulsory \
        funded retirement savings have made Australian pension funds one of the largest pools of retirement \
        savings in the world."),
    ("AU_MEDICARE",
        "Australia's universal health insurance was born twice: Medibank, created in 1975 by the Whitlam \
        government, then dismantled by its successor, and Medicare, restored in 1984 by Bob Hawke, funded \
        by this levy."),
    ("AU_INCOME_TAX",
        "In 1942, during the war, the Australian federal government took from the states the right to levy \
        income tax, “borrowing” it for the duration of the conflict. It never gave it back."),
    ("NZ_ACC",
        "The report by Justice Owen Woodhouse (1967) led in 1974 to a system unique in the world: every \
        accident victim is compensated on a no-fault basis, but in exchange gives up the right to sue the \
        person responsible."),
    ("NZ_KIWISAVER_EMP",
        "KiwiSaver was launched in 2007 by Labour Finance Minister Michael Cullen. Enrolment is automatic for \
        new employees, who can opt out: an often-cited example of a “nudge” in behavioural economics."),
];
