import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper as JsonSlurper

def response = WS.sendRequest(findTestObject('Object Repository/POST a new user', [('age') : 15, ('gender') : 'FEMALE', ('username') : 'hungtran'
            , ('password') : '123456']))

println(response)

JsonSlurper slurper = new JsonSlurper()

Map strJson = slurper.parseText(response.getResponseText())

if (strJson.status == 400) {
    println(strJson.message)
} else {		
    println(strJson)
	WS.verifyResponseStatusCode(response, 200)
}

