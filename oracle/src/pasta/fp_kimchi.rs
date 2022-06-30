use crate::poseidon::ArithmeticSpongeParams;
use mina_curves::pasta::Fp;

/* Generated by ./params.sage --rounds 55 rust 3 kimchi */

use std::str::FromStr;

pub(crate) fn params() -> ArithmeticSpongeParams<Fp> {
    ArithmeticSpongeParams {
        mds: vec![
            vec![
                Fp::from_str(
                    "12035446894107573964500871153637039653510326950134440362813193268448863222019",
                )
                .unwrap(),
                Fp::from_str(
                    "25461374787957152039031444204194007219326765802730624564074257060397341542093",
                )
                .unwrap(),
                Fp::from_str(
                    "27667907157110496066452777015908813333407980290333709698851344970789663080149",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "4491931056866994439025447213644536587424785196363427220456343191847333476930",
                )
                .unwrap(),
                Fp::from_str(
                    "14743631939509747387607291926699970421064627808101543132147270746750887019919",
                )
                .unwrap(),
                Fp::from_str(
                    "9448400033389617131295304336481030167723486090288313334230651810071857784477",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "10525578725509990281643336361904863911009900817790387635342941550657754064843",
                )
                .unwrap(),
                Fp::from_str(
                    "27437632000253211280915908546961303399777448677029255413769125486614773776695",
                )
                .unwrap(),
                Fp::from_str(
                    "27566319851776897085443681456689352477426926500749993803132851225169606086988",
                )
                .unwrap(),
            ],
        ],

        round_constants: vec![
            vec![
                Fp::from_str(
                    "21155079691556475130150866428468322463125560312786319980770950159250751855431",
                )
                .unwrap(),
                Fp::from_str(
                    "16883442198399350202652499677723930673110172289234921799701652810789093522349",
                )
                .unwrap(),
                Fp::from_str(
                    "17030687036425314703519085065002231920937594822150793091243263847382891822670",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "25216718237129482752721276445368692059997901880654047883630276346421457427360",
                )
                .unwrap(),
                Fp::from_str(
                    "9054264347380455706540423067244764093107767235485930776517975315876127782582",
                )
                .unwrap(),
                Fp::from_str(
                    "26439087121446593160953570192891907825526260324480347638727375735543609856888",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "15251000790817261169639394496851831733819930596125214313084182526610855787494",
                )
                .unwrap(),
                Fp::from_str(
                    "10861916012597714684433535077722887124099023163589869801449218212493070551767",
                )
                .unwrap(),
                Fp::from_str(
                    "18597653523270601187312528478986388028263730767495975370566527202946430104139",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "15831416454198644276563319006805490049460322229057756462580029181847589006611",
                )
                .unwrap(),
                Fp::from_str(
                    "15171856919255965617705854914448645702014039524159471542852132430360867202292",
                )
                .unwrap(),
                Fp::from_str(
                    "15488495958879593647482715143904752785889816789652405888927117106448507625751",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "19039802679983063488134304670998725949842655199289961967801223969839823940152",
                )
                .unwrap(),
                Fp::from_str(
                    "4720101937153217036737330058775388037616286510783561045464678919473230044408",
                )
                .unwrap(),
                Fp::from_str(
                    "10226318327254973427513859412126640040910264416718766418164893837597674300190",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "20878756131129218406920515859235137275859844638301967889441262030146031838819",
                )
                .unwrap(),
                Fp::from_str(
                    "7178475685651744631172532830973371642652029385893667810726019303466125436953",
                )
                .unwrap(),
                Fp::from_str(
                    "1996970955918516145107673266490486752153434673064635795711751450164177339618",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "15205545916434157464929420145756897321482314798910153575340430817222504672630",
                )
                .unwrap(),
                Fp::from_str(
                    "25660296961552699573824264215804279051322332899472350724416657386062327210698",
                )
                .unwrap(),
                Fp::from_str(
                    "13842611741937412200312851417353455040950878279339067816479233688850376089318",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "1383799642177300432144836486981606294838630135265094078921115713566691160459",
                )
                .unwrap(),
                Fp::from_str(
                    "1135532281155277588005319334542025976079676424839948500020664227027300010929",
                )
                .unwrap(),
                Fp::from_str(
                    "4384117336930380014868572224801371377488688194169758696438185377724744869360",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "21725577575710270071808882335900370909424604447083353471892004026180492193649",
                )
                .unwrap(),
                Fp::from_str(
                    "676128913284806802699862508051022306366147359505124346651466289788974059668",
                )
                .unwrap(),
                Fp::from_str(
                    "25186611339598418732666781049829183886812651492845008333418424746493100589207",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "10402240124664763733060094237696964473609580414190944671778761753887884341073",
                )
                .unwrap(),
                Fp::from_str(
                    "11918307118590866200687906627767559273324023585642003803337447146531313172441",
                )
                .unwrap(),
                Fp::from_str(
                    "16895677254395661024186292503536662354181715337630376909778003268311296637301",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "23818602699032741669874498456696325705498383130221297580399035778119213224810",
                )
                .unwrap(),
                Fp::from_str(
                    "4285193711150023248690088154344086684336247475445482883105661485741762600154",
                )
                .unwrap(),
                Fp::from_str(
                    "19133204443389422404056150665863951250222934590192266371578950735825153238612",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "5515589673266504033533906836494002702866463791762187140099560583198974233395",
                )
                .unwrap(),
                Fp::from_str(
                    "11830435563729472715615302060564876527985621376031612798386367965451821182352",
                )
                .unwrap(),
                Fp::from_str(
                    "7510711479224915247011074129666445216001563200717943545636462819681638560128",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "24694843201907722940091503626731830056550128225297370217610328578733387733444",
                )
                .unwrap(),
                Fp::from_str(
                    "27361655066973784653563425664091383058914302579694897188019422193564924110528",
                )
                .unwrap(),
                Fp::from_str(
                    "21606788186194534241166833954371013788633495786419718955480491478044413102713",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "19934060063390905409309407607814787335159021816537006003398035237707924006757",
                )
                .unwrap(),
                Fp::from_str(
                    "8495813630060004961768092461554180468161254914257386012937942498774724649553",
                )
                .unwrap(),
                Fp::from_str(
                    "27524960680529762202005330464726908693944660961000958842417927307941561848461",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "15178481650950399259757805400615635703086255035073919114667254549690862896985",
                )
                .unwrap(),
                Fp::from_str(
                    "16164780354695672259791105197274509251141405713012804937107314962551600380870",
                )
                .unwrap(),
                Fp::from_str(
                    "10529167793600778056702353412758954281652843049850979705476598375597148191979",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "721141070179074082553302896292167103755384741083338957818644728290501449040",
                )
                .unwrap(),
                Fp::from_str(
                    "22044408985956234023934090378372374883099115753118261312473550998188148912041",
                )
                .unwrap(),
                Fp::from_str(
                    "27068254103241989852888872162525066148367014691482601147536314217249046186315",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "3880429241956357176819112098792744584376727450211873998699580893624868748961",
                )
                .unwrap(),
                Fp::from_str(
                    "17387097125522937623262508065966749501583017524609697127088211568136333655623",
                )
                .unwrap(),
                Fp::from_str(
                    "6256814421247770895467770393029354017922744712896100913895513234184920631289",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "2942627347777337187690939671601251987500285937340386328746818861972711408579",
                )
                .unwrap(),
                Fp::from_str(
                    "24031654937764287280548628128490074801809101323243546313826173430897408945397",
                )
                .unwrap(),
                Fp::from_str(
                    "14401457902976567713827506689641442844921449636054278900045849050301331732143",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "20170632877385406450742199836933900257692624353889848352407590794211839130727",
                )
                .unwrap(),
                Fp::from_str(
                    "24056496193857444725324410428861722338174099794084586764867109123681727290181",
                )
                .unwrap(),
                Fp::from_str(
                    "11257913009612703357266904349759250619633397075667824800196659858304604714965",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "22228158921984425749199071461510152694025757871561406897041788037116931009246",
                )
                .unwrap(),
                Fp::from_str(
                    "9152163378317846541430311327336774331416267016980485920222768197583559318682",
                )
                .unwrap(),
                Fp::from_str(
                    "13906695403538884432896105059360907560653506400343268230130536740148070289175",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "7220714562509721437034241786731185291972496952091254931195414855962344025067",
                )
                .unwrap(),
                Fp::from_str(
                    "27608867305903811397208862801981345878179337369367554478205559689592889691927",
                )
                .unwrap(),
                Fp::from_str(
                    "13288465747219756218882697408422850918209170830515545272152965967042670763153",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "8251343892709140154567051772980662609566359215743613773155065627504813327653",
                )
                .unwrap(),
                Fp::from_str(
                    "22035238365102171608166944627493632660244312563934708756134297161332908879090",
                )
                .unwrap(),
                Fp::from_str(
                    "13560937766273321037807329177749403409731524715067067740487246745322577571823",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "21652518608959234550262559135285358020552897349934571164032339186996805408040",
                )
                .unwrap(),
                Fp::from_str(
                    "22479086963324173427634460342145551255011746993910136574926173581069603086891",
                )
                .unwrap(),
                Fp::from_str(
                    "13676501958531751140966255121288182631772843001727158043704693838707387130095",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "5680310394102577950568930199056707827608275306479994663197187031893244826674",
                )
                .unwrap(),
                Fp::from_str(
                    "25125360450906166639190392763071557410047335755341060350879819485506243289998",
                )
                .unwrap(),
                Fp::from_str(
                    "22659254028501616785029594492374243581602744364859762239504348429834224676676",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "23101411405087512171421838856759448177512679869882987631073569441496722536782",
                )
                .unwrap(),
                Fp::from_str(
                    "24149774013240355952057123660656464942409328637280437515964899830988178868108",
                )
                .unwrap(),
                Fp::from_str(
                    "5782097512368226173095183217893826020351125522160843964147125728530147423065",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "13540762114500083869920564649399977644344247485313990448129838910231204868111",
                )
                .unwrap(),
                Fp::from_str(
                    "20421637734328811337527547703833013277831804985438407401987624070721139913982",
                )
                .unwrap(),
                Fp::from_str(
                    "7742664118615900772129122541139124149525273579639574972380600206383923500701",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "1109643801053963021778418773196543643970146666329661268825691230294798976318",
                )
                .unwrap(),
                Fp::from_str(
                    "16580663920817053843121063692728699890952505074386761779275436996241901223840",
                )
                .unwrap(),
                Fp::from_str(
                    "14638514680222429058240285918830106208025229459346033470787111294847121792366",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "17080385857812672649489217965285727739557573467014392822992021264701563205891",
                )
                .unwrap(),
                Fp::from_str(
                    "26176268111736737558502775993925696791974738793095023824029827577569530708665",
                )
                .unwrap(),
                Fp::from_str(
                    "4382756253392449071896813428140986330161215829425086284611219278674857536001",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "13934033814940585315406666445960471293638427404971553891617533231178815348902",
                )
                .unwrap(),
                Fp::from_str(
                    "27054912732979753314774418228399230433963143177662848084045249524271046173121",
                )
                .unwrap(),
                Fp::from_str(
                    "28916070403698593376490976676534962592542013020010643734621202484860041243391",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "24820015636966360150164458094894587765384135259446295278101998130934963922381",
                )
                .unwrap(),
                Fp::from_str(
                    "7969535238488580655870884015145760954416088335296905520306227531221721881868",
                )
                .unwrap(),
                Fp::from_str(
                    "7690547696740080985104189563436871930607055124031711216224219523236060212249",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "9712576468091272384496248353414290908377825697488757134833205246106605867289",
                )
                .unwrap(),
                Fp::from_str(
                    "12148698031438398980683630141370402088785182722473169207262735228500190477924",
                )
                .unwrap(),
                Fp::from_str(
                    "14359657643133476969781351728574842164124292705609900285041476162075031948227",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "23563839965372067275137992801035780013422228997724286060975035719045352435470",
                )
                .unwrap(),
                Fp::from_str(
                    "4184634822776323233231956802962638484057536837393405750680645555481330909086",
                )
                .unwrap(),
                Fp::from_str(
                    "16249511905185772125762038789038193114431085603985079639889795722501216492487",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "11001863048692031559800673473526311616702863826063550559568315794438941516621",
                )
                .unwrap(),
                Fp::from_str(
                    "4702354107983530219070178410740869035350641284373933887080161024348425080464",
                )
                .unwrap(),
                Fp::from_str(
                    "23751680507533064238793742311430343910720206725883441625894258483004979501613",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "28670526516158451470169873496541739545860177757793329093045522432279094518766",
                )
                .unwrap(),
                Fp::from_str(
                    "3568312993091537758218792253361873752799472566055209125947589819564395417072",
                )
                .unwrap(),
                Fp::from_str(
                    "1819755756343439646550062754332039103654718693246396323207323333948654200950",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "5372129954699791301953948907349887257752247843844511069896766784624930478273",
                )
                .unwrap(),
                Fp::from_str(
                    "17512156688034945920605615850550150476471921176481039715733979181538491476080",
                )
                .unwrap(),
                Fp::from_str(
                    "25777105342317622165159064911913148785971147228777677435200128966844208883059",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "25350392006158741749134238306326265756085455157012701586003300872637887157982",
                )
                .unwrap(),
                Fp::from_str(
                    "20096724945283767296886159120145376967480397366990493578897615204296873954844",
                )
                .unwrap(),
                Fp::from_str(
                    "8063283381910110762785892100479219642751540456251198202214433355775540036851",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "4393613870462297385565277757207010824900723217720226130342463666351557475823",
                )
                .unwrap(),
                Fp::from_str(
                    "9874972555132910032057499689351411450892722671352476280351715757363137891038",
                )
                .unwrap(),
                Fp::from_str(
                    "23590926474329902351439438151596866311245682682435235170001347511997242904868",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "17723373371137275859467518615551278584842947963894791032296774955869958211070",
                )
                .unwrap(),
                Fp::from_str(
                    "2350345015303336966039836492267992193191479606566494799781846958620636621159",
                )
                .unwrap(),
                Fp::from_str(
                    "27755207882790211140683010581856487965587066971982625511152297537534623405016",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "6584607987789185408123601849106260907671314994378225066806060862710814193906",
                )
                .unwrap(),
                Fp::from_str(
                    "609759108847171587253578490536519506369136135254150754300671591987320319770",
                )
                .unwrap(),
                Fp::from_str(
                    "28435187585965602110074342250910608316032945187476441868666714022529803033083",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "16016664911651770663938916450245705908287192964254704641717751103464322455303",
                )
                .unwrap(),
                Fp::from_str(
                    "17551273293154696089066968171579395800922204266630874071186322718903959339163",
                )
                .unwrap(),
                Fp::from_str(
                    "20414195497994754529479032467015716938594722029047207834858832838081413050198",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "19773307918850685463180290966774465805537520595602496529624568184993487593855",
                )
                .unwrap(),
                Fp::from_str(
                    "24598603838812162820757838364185126333280131847747737533989799467867231166980",
                )
                .unwrap(),
                Fp::from_str(
                    "11040972566103463398651864390163813377135738019556270484707889323659789290225",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "5189242080957784038860188184443287562488963023922086723850863987437818393811",
                )
                .unwrap(),
                Fp::from_str(
                    "1435203288979376557721239239445613396009633263160237764653161500252258220144",
                )
                .unwrap(),
                Fp::from_str(
                    "13066591163578079667911016543985168493088721636164837520689376346534152547210",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "17345901407013599418148210465150865782628422047458024807490502489711252831342",
                )
                .unwrap(),
                Fp::from_str(
                    "22139633362249671900128029132387275539363684188353969065288495002671733200348",
                )
                .unwrap(),
                Fp::from_str(
                    "1061056418502836172283188490483332922126033656372467737207927075184389487061",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "10241738906190857416046229928455551829189196941239601756375665129874835232299",
                )
                .unwrap(),
                Fp::from_str(
                    "27808033332417845112292408673209999320983657696373938259351951416571545364415",
                )
                .unwrap(),
                Fp::from_str(
                    "18820154989873674261497645724903918046694142479240549687085662625471577737140",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "7983688435214640842673294735439196010654951226956101271763849527529940619307",
                )
                .unwrap(),
                Fp::from_str(
                    "17067928657801807648925755556866676899145460770352731818062909643149568271566",
                )
                .unwrap(),
                Fp::from_str(
                    "24472070825156236829515738091791182856425635433388202153358580534810244942762",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "25752201169361795911258625731016717414310986450004737514595241038036936283227",
                )
                .unwrap(),
                Fp::from_str(
                    "26041505376284666160132119888949817249574689146924196064963008712979256107535",
                )
                .unwrap(),
                Fp::from_str(
                    "23977050489096115210391718599021827780049209314283111721864956071820102846008",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "26678257097278788410676026718736087312816016749016738933942134600725962413805",
                )
                .unwrap(),
                Fp::from_str(
                    "10480026985951498884090911619636977502506079971893083605102044931823547311729",
                )
                .unwrap(),
                Fp::from_str(
                    "21126631300593007055117122830961273871167754554670317425822083333557535463396",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "1564862894215434177641156287699106659379648851457681469848362532131406827573",
                )
                .unwrap(),
                Fp::from_str(
                    "13247162472821152334486419054854847522301612781818744556576865965657773174584",
                )
                .unwrap(),
                Fp::from_str(
                    "8673615954922496961704442777870253767001276027366984739283715623634850885984",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "2794525076937490807476666942602262298677291735723129868457629508555429470085",
                )
                .unwrap(),
                Fp::from_str(
                    "4656175953888995612264371467596648522808911819700660048695373348629527757049",
                )
                .unwrap(),
                Fp::from_str(
                    "23221574237857660318443567292601561932489621919104226163978909845174616477329",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "1878392460078272317716114458784636517603142716091316893054365153068227117145",
                )
                .unwrap(),
                Fp::from_str(
                    "2370412714505757731457251173604396662292063533194555369091306667486647634097",
                )
                .unwrap(),
                Fp::from_str(
                    "17409784861870189930766639925394191888667317762328427589153989811980152373276",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "25869136641898166514111941708608048269584233242773814014385564101168774293194",
                )
                .unwrap(),
                Fp::from_str(
                    "11361209360311194794795494027949518465383235799633128250259863567683341091323",
                )
                .unwrap(),
                Fp::from_str(
                    "14913258820718821235077379851098720071902170702113538811112331615559409988569",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "12957012022018304419868287033513141736995211906682903915897515954290678373899",
                )
                .unwrap(),
                Fp::from_str(
                    "17128889547450684566010972445328859295804027707361763477802050112063630550300",
                )
                .unwrap(),
                Fp::from_str(
                    "23329219085372232771288306767242735245018143857623151155581182779769305489903",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "1607741027962933685476527275858938699728586794398382348454736018784568853937",
                )
                .unwrap(),
                Fp::from_str(
                    "2611953825405141009309433982109911976923326848135736099261873796908057448476",
                )
                .unwrap(),
                Fp::from_str(
                    "7372230383134982628913227482618052530364724821976589156840317933676130378411",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "20203606758501212620842735123770014952499754751430660463060696990317556818571",
                )
                .unwrap(),
                Fp::from_str(
                    "4678361398979174017885631008335559529633853759463947250620930343087749944307",
                )
                .unwrap(),
                Fp::from_str(
                    "27176462634198471376002287271754121925750749676999036165457559387195124025594",
                )
                .unwrap(),
            ],
            vec![
                Fp::from_str(
                    "6361981813552614697928697527332318530502852015189048838072565811230204474643",
                )
                .unwrap(),
                Fp::from_str(
                    "13815234633287489023151647353581705241145927054858922281829444557905946323248",
                )
                .unwrap(),
                Fp::from_str(
                    "10888828634279127981352133512429657747610298502219125571406085952954136470354",
                )
                .unwrap(),
            ],
        ],
    }
}

lazy_static::lazy_static! {
    static ref PARAMS:ArithmeticSpongeParams<Fp> = params();
}
pub fn static_parms() -> &'static ArithmeticSpongeParams<Fp> {
    &PARAMS
}
