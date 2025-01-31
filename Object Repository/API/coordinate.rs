<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>coordinate</name>
   <tag></tag>
   <elementGuidId>26bb32a2-31e1-4102-9d8e-0599d9c94191</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrlCoordinate}?q=${country}&amp;limit=5&amp;appid=${appid}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrlCoordinate</defaultValue>
      <description></description>
      <id>39d2281d-dc10-4a97-82bf-534801bdcde9</id>
      <masked>false</masked>
      <name>baseUrlCoordinate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.country</defaultValue>
      <description></description>
      <id>737fa44f-c604-4637-b83a-6c0a00c72c65</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.appid</defaultValue>
      <description></description>
      <id>d506e55a-4ed7-45c4-8940-968d67449421</id>
      <masked>false</masked>
      <name>appid</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, '[0].name', &quot;South Jakarta&quot;)
WS.verifyElementPropertyValue(response, '[0].lat', -6.28381815)
WS.verifyElementPropertyValue(response, '[0].lon', 106.80486324917382)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
