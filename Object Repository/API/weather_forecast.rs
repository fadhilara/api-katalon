<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>weather_forecast</name>
   <tag></tag>
   <elementGuidId>f5644ec1-2fae-4d7d-8da8-d4d037967a00</elementGuidId>
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
   <restUrl>${baseUrlWeatherForecast}?lat=${lat}&amp;lon=${lon}&amp;appid=${appid}</restUrl>
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
      <id>f316deda-94e3-4c25-a494-d6b040486b3e</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lon</defaultValue>
      <description></description>
      <id>465d5580-0e03-4032-aa44-ad1686cc11a2</id>
      <masked>false</masked>
      <name>lon</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.appid</defaultValue>
      <description></description>
      <id>0eb9d37a-42e6-4053-8c97-dd3c7bb4bfaf</id>
      <masked>false</masked>
      <name>appid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.baseUrlWeatherForecast</defaultValue>
      <description></description>
      <id>ee826232-1e6c-4ecf-a510-2a2b6f3ff010</id>
      <masked>false</masked>
      <name>baseUrlWeatherForecast</name>
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

WS.verifyElementPropertyValue(response, 'city.coord.lat', -6.2838)
WS.verifyElementPropertyValue(response, 'city.coord.lon', 106.8049)

WS.verifyElementPropertyValue(response, 'list[0].dt_txt', &quot;2025-01-23 12:00:00&quot;)
WS.verifyElementPropertyValue(response, 'list[0].weather[0].main', &quot;Rain&quot;)


WS.verifyElementPropertyValue(response, 'list[5].dt_txt', &quot;2025-01-24 03:00:00&quot;)
WS.verifyElementPropertyValue(response, 'list[5].weather[0].main', &quot;Clouds&quot;)

WS.verifyElementPropertyValue(response, 'list[14].dt_txt', &quot;2025-01-25 06:00:00&quot;)
WS.verifyElementPropertyValue(response, 'list[14].weather[0].main', &quot;Clouds&quot;)

WS.verifyElementPropertyValue(response, 'list[27].dt_txt', &quot;2025-01-26 21:00:00&quot;)
WS.verifyElementPropertyValue(response, 'list[27].weather[0].main', &quot;Rain&quot;)

WS.verifyElementPropertyValue(response, 'list[35].dt_txt', &quot;2025-01-27 21:00:00&quot;)
WS.verifyElementPropertyValue(response, 'list[35].weather[0].main', &quot;Rain&quot;)

WS.verifyElementPropertyValue(response, 'list[38].dt_txt', &quot;2025-01-28 06:00:00&quot;)
WS.verifyElementPropertyValue(response, 'list[38].weather[0].main', &quot;Rain&quot;)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
