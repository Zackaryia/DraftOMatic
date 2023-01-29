import requests
import time
import json

rate_limit = 4


def slicedict(d, s):
    return {k:v for k,v in d.items() if k.startswith(s)}

def get_champ_matchup_data(champ_id, role):
    print(f"Pulling data for {champ_id}, {role}")
    c1 = requests.get(f'https://axe.lolalytics.com/mega/?ep=champion&p=d&v=1&patch=30&cid={champ_id}&lane={role}&tier=all&queue=420&region=all').json()
    c2 = requests.get(f'https://axe.lolalytics.com/mega/?ep=champion2&p=d&v=1&patch=30&cid={champ_id}&lane={role}&tier=all&queue=420&region=all').json()
    time.sleep(rate_limit)


    matchup_data = {
        "analysed": c1['analysed'],
        "depth": c1['depth'],
        "n": c1['n'],
        "header": c1['header'],
        
        "enemy": slicedict(c1, "enemy_"),
        "team": slicedict(c2, "team_"),
    }

    return matchup_data

def get_all_champs_data():
    version = requests.get('https://ddragon.leagueoflegends.com/realms/na.json').json()['v']
    champions_json = requests.get(f'https://ddragon.leagueoflegends.com/cdn/{version}/data/en_US/champion.json').json()

    champ_ids = []
    for champ in champions_json['data'].values():
        champ_ids.append(int(champ['key']))

    all_champ_matchups = {}

    for champ in champ_ids:
        all_champ_matchups[champ] = {}
        for role in ['top', 'jungle', 'middle', 'bottom', 'support']:
            all_champ_matchups[champ][role] = get_champ_matchup_data(champ, role)

        with open('all_champ_data.json', 'w') as acd_raw:
            json.dump(all_champ_matchups, acd_raw)

    return all_champ_matchups

role_dict = {
    "top": 0,
    "jungle": 1,
    "middle": 2, 
    "bottom": 3,
    "support": 4
}


def get_champ_roles_played():
    all_champ_roles_that_are_played = []

    for role in ['top', 'jungle', 'middle', 'bottom', 'support']:
        x = requests.get(f'https://axe.lolalytics.com/tierlist/2/?lane={role}&patch=30&tier=all&queue=420&region=all').json()
        for (champ, values) in x['cid'].items():
            if values[0] != 0:
                if values[4]/x["totals"][values[1]]*200 > 1:

                    all_champ_roles_that_are_played.append(f"{champ}_{role_dict[role]}")
                
        time.sleep(rate_limit)

        with open("played_champ_roles.json", 'w') as json_raw:
            json.dump(all_champ_roles_that_are_played, json_raw)


get_champ_roles_played()
# get_all_champs_data()