import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import static org.assertj.core.api.Assertions.*
import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

def response = WS.sendRequestAndVerify(findTestObject("Object Repository/Login_api_1",['username':'hung011', 'password':'123456sS$']), FailureHandling.STOP_ON_FAILURE)
JsonSlurper slurper = new JsonSlurper()
Map bodyContent = slurper.parseText(response.getResponseText())

GlobalVariable.token = bodyContent.result.token
println GlobalVariable.token

response = WS.sendRequestAndVerify(findTestObject("Object Repository/GetProjectInject"))

println response

JsonSlurper slurper1 = new JsonSlurper()
bodyContent = slurper1.parseText(response.getResponseText())

println bodyContent.status
println bodyContent.success
println bodyContent.status
println bodyContent.result.size()

for(item in bodyContent.result) {
	println item._id
	println item.projectname
	println item.description
	println item.status
	println item.role
}

println bodyContent
