import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

response = WS.sendRequest(findTestObject('API/polution'))

WS.verifyResponseStatusCode(response, 200)

WS.containsString(response, 'co', false)

WS.containsString(response, 'no2', false)

WS.containsString(response, 'o3', false)

WS.containsString(response, 'so2', false)

WS.containsString(response, 'pm2_5', false)

WS.containsString(response, 'pm10', false)

WS.containsString(response, 'nh3', false)

String jsonSchema =
"""
{
  "\$id": "https://example.com/person.schema.json",
  "\$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "coord": {
      "type": "object",
      "properties": {
        "lon": {
          "type": "number"
        },
        "lat": {
          "type": "number"
        }
      },
      "required": [
        "lon",
        "lat"
      ]
    },
    "list": {
      "type": "array",
      "items": [
        {
          "type": "object",
          "properties": {
            "main": {
              "type": "object",
              "properties": {
                "aqi": {
                  "type": "integer"
                }
              },
              "required": [
                "aqi"
              ]
            },
            "components": {
              "type": "object",
              "properties": {
                "co": {
                  "type": "number"
                },
                "no": {
                  "type": "number"
                },
                "no2": {
                  "type": "number"
                },
                "o3": {
                  "type": "number"
                },
                "so2": {
                  "type": "number"
                },
                "pm2_5": {
                  "type": "number"
                },
                "pm10": {
                  "type": "number"
                },
                "nh3": {
                  "type": "number"
                }
              },
              "required": [
                "co",
                "no",
                "no2",
                "o3",
                "so2",
                "pm2_5",
                "pm10",
                "nh3"
              ]
            },
            "dt": {
              "type": "integer"
            }
          },
          "required": [
            "main",
            "components",
            "dt"
          ]
        }
      ]
    }
  },
  "required": [
    "coord",
    "list"
  ]
}
"""

boolean successful = WS.validateJsonAgainstSchema(response,jsonSchema)

