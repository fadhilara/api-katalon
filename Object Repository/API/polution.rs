<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>polution</name>
   <tag></tag>
   <elementGuidId>0b1ed402-697c-4818-bf64-8b00cca2bbb0</elementGuidId>
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
   <restUrl>${baseUrlPolution}?lat=${lat}&amp;lon=${lon}&amp;appid=${appid}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.lat</defaultValue>
      <description></description>
      <id>f92eea0d-80f0-4b7c-ae57-38f9f87bd32e</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lon</defaultValue>
      <description></description>
      <id>2aaf5b1e-dc90-4151-9ce7-baea7d5ddc33</id>
      <masked>false</masked>
      <name>lon</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.appid</defaultValue>
      <description></description>
      <id>b0a7f6b8-b50a-4a45-868b-5e770e9b580f</id>
      <masked>false</masked>
      <name>appid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.baseUrlPolution</defaultValue>
      <description></description>
      <id>e4b114d2-3e96-456c-9fdc-391f0513aeb4</id>
      <masked>false</masked>
      <name>baseUrlPolution</name>
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

WS.verifyElementPropertyValue(response, 'coord.lon', 106.8049)
WS.verifyElementPropertyValue(response, 'coord.lat', -6.2838)

WS.verifyElementPropertyValue(response, 'list[0].main.aqi', 5)

WS.verifyElementPropertyValue(response, 'list[0].components.co', 6301.88)
WS.verifyElementPropertyValue(response, 'list[0].components.no', 70.63)
WS.verifyElementPropertyValue(response, 'list[0].components.no2', 102.82)
WS.verifyElementPropertyValue(response, 'list[0].components.03', 0.44)
WS.verifyElementPropertyValue(response, 'list[0].components.so2', 55.31)
WS.verifyElementPropertyValue(response, 'list[0].components.pm2_5', 100.86)
WS.verifyElementPropertyValue(response, 'list[0].components.pm10', 131.35)
WS.verifyElementPropertyValue(response, 'list[0].components.nh3', 13.05)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
