import unittest
import requests
import helper as helper
import models
import json
    
maxDiff = None
#host = "http://aws3.proteininformationresource.org"
host = "http://localhost:8088"


# test get info
def test_get_info():

    url = "{host}/Q15796/info".format(host=host)

    result = requests.get(url)

    # assert that the request succeeded
    assert result.status_code == 200, result.content

    # parse the returned response
    returned_info = json.loads(result.text)

    # load the expected response
    expected_info = helper.load_json("info.json")

    # assert if proper response is returned
    assert expected_info == returned_info

# test search json
def test_search():
    params = {
        "search_term": "smad2",
        "term_type": "All",
        "ptm_type": ["Acetylation",
                        "C-Glycosylation",
                        "Myristoylation",
                        "Ubiquitination",
                        "N-Glycosylation",
                        "S-Glycosylation",
                        "Phosphorylation",
                        "S-Nitrosylation",
                        "O-Glycosylation",
                        "Methylation",
                        "Sumoylation"],
        "role": "Enzyme or Substrate",
        "organism": []
    }

    result = requests.get('{host}/search'.format(host=host),params = params)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # assert if header contains count
    assert result.headers.get("count") != None

    # parse the returned response
    returned_search_results = json.loads(result.text)

    # load the expected response
    expected_search_results = helper.load_json("search.json")

    for index, search_result in enumerate(expected_search_results):
        assert (search_result in returned_search_results) == True, "Item at index: {index} not found".format(index=index)
        

# test search json csv
def test_search_csv():
    params = {
        "search_term": "smad2",
        "term_type": "All",
        "ptm_type": ["Acetylation",
                        "C-Glycosylation",
                        "Myristoylation",
                        "Ubiquitination",
                        "N-Glycosylation",
                        "S-Glycosylation",
                        "Phosphorylation",
                        "S-Nitrosylation",
                        "O-Glycosylation",
                        "Methylation",
                        "Sumoylation"],
        "role": "Enzyme or Substrate",
        "organism": [10090,9606]
    }

    headers = {
        "Accept": "text/plain"
    }

    result = requests.get('{host}/search'.format(host=host),params = params,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # assert if header contains count
    assert result.headers.get("count") is not None

    # parse the returned response
    returned_search_results = helper.load_csv_from_string(result.text)

    # load the expected response
    expected_search_results = helper.load_csv_from_file("search.csv")

    for index,search_result in enumerate(expected_search_results):
        assert (search_result in returned_search_results) == True, "Item at index: {index} not found".format(index=index)


# test browse json
def test_browse():
    params = {
        "term_type": "All",
        "start_index": "0",
        "end_index": "120",
        "ptm_type": ["Acetylation",
                        "C-Glycosylation",
                        "Myristoylation",
                        "Ubiquitination",
                        "N-Glycosylation",
                        "S-Glycosylation",
                        "Phosphorylation",
                        "S-Nitrosylation",
                        "O-Glycosylation",
                        "Methylation",
                        "Sumoylation"],
        "role": "Enzyme or Substrate",
        "organism": []
    }

    result = requests.get('{host}/browse'.format(host=host),params = params)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # assert if header contains count
    assert result.headers.get("count") is not None

    # parse the returned response
    returned_search_results = json.loads(result.text)

    assert len(returned_search_results) == 120

# test search json csv
def test_browse_csv():
    params = {
        "term_type": "All",
        "start_index": "0",
        "end_index": "120",
        "ptm_type": ["Acetylation",
                        "C-Glycosylation",
                        "Myristoylation",
                        "Ubiquitination",
                        "N-Glycosylation",
                        "S-Glycosylation",
                        "Phosphorylation",
                        "S-Nitrosylation",
                        "O-Glycosylation",
                        "Methylation",
                        "Sumoylation"],
        "role": "Enzyme or Substrate",
        "organism": [10090,9606]
    }

    headers = {
        "Accept": "text/plain"
    }

    result = requests.get('{host}/browse'.format(host=host),params = params,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # assert if header contains count
    assert result.headers.get("count") is not None

    # parse the returned response
    returned_search_results = helper.load_csv_from_string(result.text)

    assert len(returned_search_results) == 120

# test get substrate json
def test_get_substrates():

    url = "{host}/Q15796/substrate".format(host=host)

    result = requests.get(url)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_substrate = json.loads(result.text)

    # assert that returned response is not empty
    assert len(returned_substrate.keys()) is not 0               


def test_substrate_csv():
    headers = {
        "Accept": "text/plain",
        "Origin": host
    }

    result = requests.get('{host}/Q15796/substrate'.format(host=host),headers=headers)

    # assert response == Ok
    assert result.status_code ==200,result.text

    # parse the result
    returned_substrates = helper.load_csv_from_string(result.text)
    helper.sanitize_substrates(returned_substrates)

    # assert that returned response is not empty
    assert len(returned_substrates) is not 0     

# test get proteoforms json
def test_get_proteoforms():

    url = "{host}/Q15796/proteoforms".format(host=host)

    result = requests.get(url)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_proteoforms = json.loads(result.text)
    
    # assert that returned response is not empty
    assert len(returned_proteoforms) is not 0         

# test proteoform csv
def test_proteoform_csv():

    headers = {
        "Accept": "text/plain"
    }

    result = requests.get(url="{host}/Q15796/proteoforms".format(host=host),headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_proteoforms = helper.load_csv_from_string(result.text)

    # assert that returned response is not empty
    assert len(returned_proteoforms) is not 0   

# test get proteoformsppi json
def test_get_proteoformsppi():

    url = "{host}/Q15796/proteoformsppi".format(host=host)

    result = requests.get(url)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_proteoformsppi = json.loads(result.text)

    # assert that returned response is not empty
    assert len(returned_proteoformsppi) is not 0   

# test get proteoformsppi csv
def test_get_proteoformsppi_csv():

    url = "{host}/Q15796/proteoformsppi".format(host=host)

    headers = {
        "Accept": "text/plain"
    }

    result = requests.get(url=url,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_proteoformsppi = helper.load_csv_from_string(result.text)

    # assert that returned response is not empty
    assert len(returned_proteoformsppi) is not 0 

def test_get_ptmppi():
    url = "{host}/Q15796/ptmppi".format(host=host)

    result = requests.get(url)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_ptmppi = json.loads(result.text)

    # assert that returned response is not empty
    assert len(returned_ptmppi) is not 0 

def test_get_ptmppi_csv():
    url = "{host}/Q15796/ptmppi".format(host=host)

    headers = {
        "Accept": "text/plain"
    }

    result = requests.get(url=url,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_ptmppis = helper.load_csv_from_string(result.text)

    # assert that returned response is not empty
    assert len(returned_ptmppis) is not 0 

# test batch ptm enzymes
def test_batch_ptm_enzymes_json():

    # construct the body
    substrates = [models.QuerySubstrate("Q15796", "K", "19"),
                    models.QuerySubstrate("Q15796", "T", "8"),
                    models.QuerySubstrate("P04637", "K", "120"),
                    models.QuerySubstrate("P04637", "S", "149"),
                    models.QuerySubstrate("P04637", "S", "378"),
                    models.QuerySubstrate("P04637", "S", "392"),
                    models.QuerySubstrate("P42356", "S", "199")]

    body_str = json.dumps(substrates,default=lambda o:o.__dict__,indent=4, sort_keys=True)

    headers = {
        "Accept": "application/json"
    }

    url = "{host}/batch_ptm_enzymes".format(host=host)
    result = requests.post(url=url,data=body_str,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_enzymes = json.loads(result.text)

    # assert that returned response is not empty
    assert len(returned_enzymes) is not 0 


# test batch ptm enzymes
def test_batch_ptm_enzymes_csv():

    # construct the body
    substrates = [models.QuerySubstrate("Q15796", "K", "19"),
                    models.QuerySubstrate("Q15796", "T", "8"),
                    models.QuerySubstrate("P04637", "K", "120"),
                    models.QuerySubstrate("P04637", "S", "149"),
                    models.QuerySubstrate("P04637", "S", "378"),
                    models.QuerySubstrate("P04637", "S", "392"),
                    models.QuerySubstrate("P42356", "S", "199")]

    body_str = json.dumps(substrates,default=lambda o:o.__dict__,indent=4, sort_keys=True)

    headers = {
        "Accept": "text/plain"
    }

    url = "{host}/batch_ptm_enzymes".format(host=host)
    result = requests.post(url=url,data=body_str,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_enzymes = helper.load_csv_from_string(result.text)

    # assert that returned response is not empty
    assert len(returned_enzymes) is not 0 


# test batch ptm ppi json
def test_batch_ptm_ppi_json():

    # construct the body
    substrates = [models.QuerySubstrate("Q15796", "K", "19"),
                    models.QuerySubstrate("Q15796", "T", "8"),
                    models.QuerySubstrate("P04637", "K", "120"),
                    models.QuerySubstrate("P04637", "S", "149"),
                    models.QuerySubstrate("P04637", "S", "378"),
                    models.QuerySubstrate("P04637", "S", "392"),
                    models.QuerySubstrate("P42356", "S", "199")]

    body_str = json.dumps(substrates,default=lambda o:o.__dict__,indent=4, sort_keys=True)

    headers = {
        "Accept": "application/json"
    }

    result = requests.post(url='{host}/batch_ptm_ppi'.format(host=host),data=body_str,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_ptmppi = json.loads(result.text)

    # assert that returned response is not empty
    assert len(returned_ptmppi) is not 0 

# test batch ptm ppi csv
def test_batch_ptm_ppi_csv():

    # construct the body
    substrates = [models.QuerySubstrate("Q15796", "K", "19"),
                    models.QuerySubstrate("Q15796", "T", "8"),
                    models.QuerySubstrate("P04637", "K", "120"),
                    models.QuerySubstrate("P04637", "S", "149"),
                    models.QuerySubstrate("P04637", "S", "378"),
                    models.QuerySubstrate("P04637", "S", "392"),
                    models.QuerySubstrate("P42356", "S", "199")]

    body_str = json.dumps(substrates,default=lambda o:o.__dict__,indent=4, sort_keys=True)

    headers = {
        "Accept": "text/plain"
    }

    result = requests.post(url='{host}/batch_ptm_ppi'.format(host=host),data=body_str,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_ptmppi = helper.load_csv_from_string(result.text)

    # assert that returned response is not empty
    assert len(returned_ptmppi) is not 0 

# test batch ptm ppi csv
def test_msa():

    headers = {
        "Accept": "application/json"
    }

    result = requests.get(url='{host}/Q15796/msa'.format(host=host),headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_msa = json.loads(result.text)

    # assert
    assert len(returned_msa) == 3

# test get_variants_json 
def test_get_variants():

    url = "{host}/Q15796/variants".format(host=host)

    result = requests.get(url)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_variants = json.loads(result.text)

    # assert that returned response is not empty
    assert len(returned_variants) is not 0   

# test get proteoformsppi csv
def test_get_proteoformsppi_csv():

    url = "{host}/Q15796/variants".format(host=host)

    headers = {
        "Accept": "text/plain"
    }

    result = requests.get(url=url,headers=headers)

    # assert if request was successful
    assert result.status_code == 200, result.text

    # parse the returned response
    returned_variants = helper.load_csv_from_string(result.text)

    # assert that returned response is not empty
    assert len(returned_variants) is not 0 