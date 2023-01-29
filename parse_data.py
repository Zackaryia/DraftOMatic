

import json, requests, math

def adj(games):
    if games == None:
        return 1
    
    # https://www.desmos.com/calculator/iptdekmcs8
    a = 0.6
    b = 10
    c = 10  
    
    if games < c:
        return 0

    return (a*(games-c))/(a*(games-c)+b)


def w(r):
    print("R", r)
    return 100/(1+math.pow(10, -r/400))

def r(w, games=None):
    if w < 0.5:
        w = 0.5
    elif w > 99.5:
        w = 99.5

    return (-400*math.log10(100/w-1))*adj(games)


def duo(awr, ewr1, ewr2, games=None):
    return (r(awr) - (r(ewr1) + r(ewr2)))*adj(games)

def vs(awr, ewr1, ewr2, games=None): 
    return r(awr) - (r(ewr1) - r(ewr2))*adj(games)
             


with open("all_champ_data.json") as ej:
    data = json.load(ej)

    version = requests.get('https://ddragon.leagueoflegends.com/realms/na.json').json()['v']
    champions_json = requests.get(f'https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json').json()

    champ_ids = []
    for champ in champions_json['data'].values():
        champ_ids.append(int(champ['key']))


    matchup_data = {}
    champion_dmg_data = {}
    champion_r_data = {}

    role_dict = {
        "top": 0,
        "jungle": 1,
        "middle": 2, 
        "bottom": 3,
        "support": 4
    }

    for champ, value in data.items():
        for role, cr_data in value.items():
            champion_r_data[f"champr_{champ}_{role_dict[role]}"] = r(cr_data['header']['wr'], cr_data['n'])
            champion_dmg_data[f"champd_{champ}_{role_dict[role]}"] = list(cr_data['header']['damage'].values())
            for hostility, value in [("team", cr_data['team']), ('enemy', cr_data['enemy'])]:
                for vs_role, matchups in value.items():
                    for matchup in matchups:
                        print(f"{champ}_{role_dict[role]}_{matchup[0]}_{role_dict[vs_role.split('_')[1]]}")
                        if hostility == "team":
                            matchup_data[f"team_{champ}_{role_dict[role]}_{matchup[0]}_{role_dict[vs_role.split('_')[1]]}"] = duo(100*(matchup[2]/matchup[1]), cr_data['header']['wr'], matchup[3], matchup[1])
                        if hostility == "enemy":
                            matchup_data[f"enemy_{champ}_{role_dict[role]}_{matchup[0]}_{role_dict[vs_role.split('_')[1]]}"] = vs(100*(matchup[2]/matchup[1]), cr_data['header']['wr'], matchup[3], matchup[1])

with open("matchup_data.json", 'w') as json_raw:
    json.dump(matchup_data, json_raw)
with open("champion_dmg_data.json", 'w') as json_raw:
    json.dump(champion_dmg_data, json_raw)
with open("champion_r_data.json", 'w') as json_raw:
    json.dump(champion_r_data, json_raw)
with open("champ_ids.json", 'w') as json_raw:
    json.dump(champ_ids, json_raw)