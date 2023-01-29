from flask import Flask, current_app, render_template, request
from draftomatic import evaluate
import json

app = Flask(__name__)
# evaluate([], 0)

tot_analyzed = 0

@app.route("/<state>/<depth>")
def get_draft_eval(state, depth):

    return evaluate(draft, int(depth))
    

@app.route("/")
def home():
    print("123")
    with open("view.html", 'r') as fraw:
        return fraw.read()
    
@app.route("/d/<draft>/")
def draft_r(draft):
    global tot_analyzed
    if draft == ",":
        draft = []
    else:
        draft = [int(x) for x in draft.split(',')]
    
    depth = request.args.get('depth')

    if depth is None:
        depth = 3

    evalu = json.loads(evaluate(draft, depth))
    tot_analyzed += evalu[1]

    return render_template('view.html', draft=draft, e=evalu, Depth=depth, Nodes_Analyzed=tot_analyzed)

    
@app.route("/file/<file_name>")
def files(file_name):
    print("123")
    with open(file_name, 'r') as fraw:
        return fraw.read()

app.run(debug=True)