import unittest
import requests
import test_helper as helper
import models
import json

class TestApplication(unittest.TestCase):
    maxDiff = None
    #host = "http://aws3.proteininformationresource.org"
    host = "http://localhost:8088"


    # test get info
    def test_get_info(self):

        url = "{host}/Q15796/info".format(host=self.host)

        result = requests.get(url)

        # assert that the request succeeded
        self.assertEqual(result.status_code,200,result.content)

        # parse the returned response
        returned_info = json.loads(result.text)

        # load the expected response
        expected_info = helper.load_json("info.json")

        # assert if proper response is returned
        self.assertEqual(expected_info,returned_info)

    # test search json
    def test_search(self):
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

        result = requests.get('{host}/search'.format(host=self.host),params = params)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # assert if header contains count
        self.assertIsNotNone(result.headers.get("count"))

        # parse the returned response
        returned_search_results = json.loads(result.text)

        # load the expected response
        expected_search_results = helper.load_json("search.json")

        for index, search_result in enumerate(expected_search_results):
            self.assertEqual(search_result in returned_search_results, True, "Item at index: {index} not found".format(index=index))
            

    # test search json csv
    def test_search_csv(self):
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

        result = requests.get('{host}/search'.format(host=self.host),params = params,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # assert if header contains count
        self.assertIsNotNone(result.headers.get("count"))

        # parse the returned response
        returned_search_results = helper.load_csv_from_string(result.text)

        # load the expected response
        expected_search_results = helper.load_csv_from_file("search.csv")

        for index,search_result in enumerate(expected_search_results):
            self.assertEqual(search_result in returned_search_results, True, "Item at index: {index} not found".format(index=index))


    # test browse json
    def test_browse(self):
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

        result = requests.get('{host}/browse'.format(host=self.host),params = params)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # assert if header contains count
        self.assertIsNotNone(result.headers.get("count"))

        # parse the returned response
        returned_search_results = json.loads(result.text)

        self.assertEqual(len(returned_search_results),120)

    # test search json csv
    def test_browse_csv(self):
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

        result = requests.get('{host}/browse'.format(host=self.host),params = params,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # assert if header contains count
        self.assertIsNotNone(result.headers.get("count"))

        # parse the returned response
        returned_search_results = helper.load_csv_from_string(result.text)

        self.assertEqual(len(returned_search_results), 120)

    # test get substrate json
    def test_get_substrates(self):

        url = "{host}/Q15796/substrate".format(host=self.host)

        result = requests.get(url)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_substrate = json.loads(result.text)

        # load the expected response
        expected_substrate = helper.load_json("substrate.json")

        for index,substrate in enumerate(expected_substrate):
            self.assertEqual(substrate in returned_substrate, True,"Item at {index} not found".format(index=index))


    def test_substrate_csv(self):
        headers = {
            "Accept": "text/plain",
            "Origin": self.host
        }

        result = requests.get('{host}/Q15796/substrate'.format(host=self.host),headers=headers)

        # assert response == Ok
        self.assertEqual(result.status_code,200,result.text)

        # parse the result
        returned_substrates = helper.load_csv_from_string(result.text)
        helper.sanitize_substrates(returned_substrates)

        # load expected substrates
        expected_substrates = helper.load_csv_from_file("substrate.csv")
        helper.sanitize_substrates(expected_substrates)

        # assert
        for index,substrate in enumerate(expected_substrates):
            self.assertEqual(substrate in returned_substrates, True,"Item at {index} not found".format(index=index))

    # test get proteoforms json
    def test_get_proteoforms(self):

        url = "{host}/Q15796/proteoforms".format(host=self.host)

        result = requests.get(url)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_proteoforms = json.loads(result.text)

        # load the expected response
        expected_proteoforms = helper.load_json("proteoforms.json")

        for index,proteoform in enumerate(expected_proteoforms):
            self.assertEqual(proteoform in returned_proteoforms, True,"Item at {index} not found".format(index=index))

    # test proteoform csv
    def test_proteoform_csv(self):

        headers = {
            "Accept": "text/plain"
        }

        result = requests.get(url="{host}/Q15796/proteoforms".format(host=self.host),headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_proteoforms = helper.load_csv_from_string(result.text)

        # load the expected response
        expected_proteoforms = helper.load_csv_from_file("proteoforms.csv")

        for index,proteoform in enumerate(expected_proteoforms):
            self.assertEqual(proteoform in returned_proteoforms, True,"Item at {index} not found".format(index=index))

    # test get proteoformsppi json
    def test_get_proteoformsppi(self):

        url = "{host}/Q15796/proteoformsppi".format(host=self.host)

        result = requests.get(url)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_proteoformsppi = json.loads(result.text)

        # load the expected response
        expected_proteoformsppi = helper.load_json("proteoformsppi.json")

        for index,proteoformppi in enumerate(expected_proteoformsppi):
            self.assertEqual(proteoformppi in returned_proteoformsppi, True, "Item at {index} not found".format(index=index))


    # test get proteoformsppi csv
    def test_get_proteoformsppi_csv(self):

        url = "{host}/Q15796/proteoformsppi".format(host=self.host)

        headers = {
            "Accept": "text/plain"
        }

        result = requests.get(url=url,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_proteoformsppi = helper.load_csv_from_string(result.text)

        # load the expected response
        expected_proteoformsppi = helper.load_csv_from_file("proteoformsppi.csv")

        for index,proteoformppi in enumerate(expected_proteoformsppi):
            self.assertEqual(proteoformppi in returned_proteoformsppi, True, "Item at {index} not found".format(index=index))

    def test_get_ptmppi(self):
        url = "{host}/Q15796/ptmppi".format(host=self.host)

        result = requests.get(url)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_ptmppi = json.loads(result.text)

        # load the expected response
        expected_ptmppi = helper.load_json("ptmppi.json")

        for index,ptmppi in enumerate(expected_ptmppi):
            self.assertEqual(ptmppi in returned_ptmppi, True, "Item at {index} not found".format(index=index))


    def test_get_ptmppi_csv(self):
        url = "{host}/Q15796/ptmppi".format(host=self.host)

        headers = {
            "Accept": "text/plain"
        }

        result = requests.get(url=url,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_ptmppis = helper.load_csv_from_string(result.text)

        # load the expected response
        expected_ptmppis = helper.load_csv_from_file("ptmppi.csv")

        for index,ptmppi in enumerate(expected_ptmppis):
            self.assertEqual(ptmppi in returned_ptmppis, True, "Item at {index} not found".format(index=index))

    # test batch ptm enzymes
    def test_batch_ptm_enzymes_json(self):

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

        url = "{host}/batch_ptm_enzymes".format(host=self.host)
        result = requests.post(url=url,data=body_str,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_enzymes = json.loads(result.text)

        # load the expected response
        expected_enzymes = helper.load_json("batch_ptm_enzymes.json")

        for index,enzyme in enumerate(expected_enzymes):
            self.assertEqual(enzyme in returned_enzymes, True, "Item at {index} not found".format(index=index))


    # test batch ptm enzymes
    def test_batch_ptm_enzymes_csv(self):

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

        url = "{host}/batch_ptm_enzymes".format(host=self.host)
        result = requests.post(url=url,data=body_str,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_enzymes = helper.load_csv_from_string(result.text)

        # load the expected response
        expected_enzymes = helper.load_csv_from_file("batch_ptm_enzymes.csv")

        for index,enzyme in enumerate(expected_enzymes):
            self.assertEqual(enzyme in returned_enzymes, True, "Item at {index} not found".format(index=index))


    # test batch ptm ppi json
    def test_batch_ptm_ppi_json(self):

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

        result = requests.post(url='{host}/batch_ptm_ppi'.format(host=self.host),data=body_str,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_ptmppi = json.loads(result.text)

        # load the expected response
        expected_ptmppi = helper.load_json("batch_ptm_ppi.json")

        for index,ptmppi in enumerate(expected_ptmppi):
            self.assertEqual(ptmppi in returned_ptmppi, True, "Item at {index} not found".format(index=index))

    # test batch ptm ppi csv
    def test_batch_ptm_ppi_csv(self):

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

        result = requests.post(url='{host}/batch_ptm_ppi'.format(host=self.host),data=body_str,headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_ptmppi = helper.load_csv_from_string(result.text)

        # load the expected response
        expected_ptmppi = helper.load_csv_from_file("batch_ptm_ppi.csv")

        for index,ptmppi in enumerate(expected_ptmppi):
            self.assertEqual(ptmppi in returned_ptmppi, True, "Item at {index} not found".format(index=index))

    # test batch ptm ppi csv
    def test_msa(self):

        headers = {
            "Accept": "application/json"
        }

        result = requests.get(url='{host}/Q15796/msa'.format(host=self.host),headers=headers)

        # assert if request was successful
        self.assertEqual(result.status_code, 200, result.text)

        # parse the returned response
        returned_msa = helper.load_csv_from_string(result.text)

        # assert
        self.assertEqual(len(returned_msa),3)


if __name__ == '__main__':
    unittest.main()
