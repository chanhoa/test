<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST a new user_usingVariable</name>
   <tag></tag>
   <elementGuidId>31c98e02-86e3-41c3-a25f-eeccb3437fe1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;age\&quot;: ${age},\n  \&quot;avatar\&quot;: null,\n  \&quot;gender\&quot;:\&quot;${gender}\&quot;,\n  \&quot;password\&quot;: \&quot;${password}\&quot;,\n  \&quot;username\&quot;: \&quot;${username}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/users/json</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>18</defaultValue>
      <description></description>
      <id>0ff0fdb5-ffe2-454e-8cf9-c2d44e21ddac</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <variables>
      <defaultValue>'MALE'</defaultValue>
      <description></description>
      <id>7c77cd5e-e69d-4bf6-bf4c-242ca104081c</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>'mimi'</defaultValue>
      <description></description>
      <id>30565e15-eece-49b1-8159-ac9a99a956aa</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'123456789'</defaultValue>
      <description></description>
      <id>303e8db4-748d-4300-878d-05ef1956d9b6</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
assert response.getStatusCode() == 200
WS.verifyElementPropertyValue(response, &quot;age&quot;, 25)
WS.verifyElementPropertyValue(response, &quot;username&quot;, &quot;mimi&quot;)
WS.verifyElementPropertyValue(response, &quot;password&quot;, &quot;123456789&quot;)
WS.verifyElementPropertyValue(response, &quot;gender&quot;, &quot;MALE&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
