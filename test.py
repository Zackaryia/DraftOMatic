z = {"1":"Annie","2":"Olaf","3":"Galio","4":"TwistedFate","5":"XinZhao","6":"Urgot","7":"Leblanc","8":"Vladimir","9":"Fiddlesticks","10":"Kayle","11":"MasterYi","12":"Alistar","13":"Ryze","14":"Sion","15":"Sivir","16":"Soraka","17":"Teemo","18":"Tristana","19":"Warwick","20":"Nunu","21":"MissFortune","22":"Ashe","23":"Tryndamere","24":"Jax","25":"Morgana","26":"Zilean","27":"Singed","28":"Evelynn","29":"Twitch","30":"Karthus","31":"Chogath","32":"Amumu","33":"Rammus","34":"Anivia","35":"Shaco","36":"DrMundo","37":"Sona","38":"Kassadin","39":"Irelia","40":"Janna","41":"Gangplank","42":"Corki","43":"Karma","44":"Taric","45":"Veigar","48":"Trundle","50":"Swain","51":"Caitlyn","53":"Blitzcrank","54":"Malphite","55":"Katarina","56":"Nocturne","57":"Maokai","58":"Renekton","59":"JarvanIV","60":"Elise","61":"Orianna","62":"MonkeyKing","63":"Brand","64":"LeeSin","67":"Vayne","68":"Rumble","69":"Cassiopeia","72":"Skarner","74":"Heimerdinger","75":"Nasus","76":"Nidalee","77":"Udyr","78":"Poppy","79":"Gragas","80":"Pantheon","81":"Ezreal","82":"Mordekaiser","83":"Yorick","84":"Akali","85":"Kennen","86":"Garen","89":"Leona","90":"Malzahar","91":"Talon","92":"Riven","96":"KogMaw","98":"Shen","99":"Lux","101":"Xerath","102":"Shyvana","103":"Ahri","104":"Graves","105":"Fizz","106":"Volibear","107":"Rengar","110":"Varus","111":"Nautilus","112":"Viktor","113":"Sejuani","114":"Fiora","115":"Ziggs","117":"Lulu","119":"Draven","120":"Hecarim","121":"Khazix","122":"Darius","126":"Jayce","127":"Lissandra","131":"Diana","133":"Quinn","134":"Syndra","136":"AurelionSol","141":"Kayn","142":"Zoe","143":"Zyra","145":"Kaisa","147":"Seraphine","150":"Gnar","154":"Zac","157":"Yasuo","161":"Velkoz","163":"Taliyah","164":"Camille","166":"Akshan","200":"Belveth","201":"Braum","202":"Jhin","203":"Kindred","221":"Zeri","222":"Jinx","223":"TahmKench","234":"Viego","235":"Senna","236":"Lucian","238":"Zed","240":"Kled","245":"Ekko","246":"Qiyana","254":"Vi","266":"Aatrox","267":"Nami","268":"Azir","350":"Yuumi","360":"Samira","412":"Thresh","420":"Illaoi","421":"RekSai","427":"Ivern","429":"Kalista","432":"Bard","497":"Rakan","498":"Xayah","516":"Ornn","517":"Sylas","518":"Neeko","523":"Aphelios","526":"Rell","555":"Pyke","711":"Vex","777":"Yone","875":"Sett","876":"Lillia","887":"Gwen","888":"Renata","895":"Nilah","897":"KSante"}
roles = ["top", "jgl", "mid", "bot", "sup"]

def crpring(cr):
    print(z[str(cr & 0b111111111111)], roles[cr >> 12])
    print(str(cr & 0b111111111111) + "_" + str(cr >> 12))
    print()

x = """4342
4229
4182
4125
4260
64
76
4150
44
4334
427
8273
12292
8268
12330
59
4119
8460
4194
421
4160
236
4164
897
12523
234
136
8199
4176
12811
4171
8428
4127
4203
11
12422
3
85
84
246
4187
4216
4983
4172
79
8438
517
35
68
429
9089
268
157
12717
42
48
12454
8256
8334
4218
8413
4259
150
266
8234
5
221
58
12369
106
12305
8253
92
8
104
12806
4517
12524
777
4237
30
8276
4126
45
887
8302
4098
4299
1
8196
8709
4330
4200
4155
8430
8328
4174
127
876
56
4217
12421
8326
8247
12509
12449
8283
80
23
8335
4131
4113
9068
9079
43
8295
8747
8251
12451
31
4175
8222
78
67
4523
223
4613
8193
8355
12355
122
8205
4144
12407
13
8349
50
12303
4101
8319
12511
39
102
10
126
12433
12351
164
8271
4110
8415
41
12510
54
12339
8237
12445
12398
8288
4120
8969
33
12310
112
17
8621
166
107
8200
4124
4972
113
8260
8426
8293
8217
8195
8284
8339
90
8458
8437
4296
4107
4202
12786
8710
8307
57
114
8323
8358
38
14
8297
4209
6
8248
8202
12317
875
8233
34
4123
8277
516
62
518
8304
133
4227
8223
12384
24
8255
8903
4341
12333
8619
2
12309
8231
4140
8296
8216
12306
4158
8198
8246
74
19
8235
8353
98
4198
4129
8250
4115
69
4156
8209
8291
86
8261
82
77
12338
8215
8197
4350
8272
8242
12490
420
12648
240
12403
75
4105
83
8221
8325
8230
8708
8210
8282
8356
12357
4152
8318
8206
8254
27
4173
4132
4250
8201
12362
8228
8218
12318
8275
8267
12435
9067
4178
13183
8226
36
154
8266
8278
8274
8612
8294
4153
4128
8290
8346
8432
8219"""

for row in x.split("\n"):
    crpring(int(row))

print("\n")

crpring(8264)
crpring(4342)
crpring(16404)
crpring(16520)

print("_---")

print(crpring(154))
print(crpring(3))
