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
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject

WebUI.callTestCase(findTestCase('Reclutador/login/login Admin'), [:], FailureHandling.STOP_ON_FAILURE)

println('se hace login')

response = WS.sendRequest(findTestObject('Reclutador/restablecer pass/usuarios'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

responseText = response.getResponseText()

println(responseText)

def json = new JsonSlurper().parseText(responseText)

println(json.content.email)

println(json.size)

RN = json.size()

println(RN)

aleatoreo()

println(rn)

emails = json.content.email

GlobalVariable.email = (emails[rn])

println(GlobalVariable.email)

println('se obtienen los usuarios')

response = WS.sendRequest(findTestObject('Reclutador/restablecer pass/restablecer pass'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

WebUI.openBrowser('https://yopmail.com/es/wm')

WebUI.setText(findTestObject('Generales/Cuenta Bloqueada/Campo Email'), GlobalVariable.email)

WebUI.sendKeys(findTestObject('Generales/Cuenta Bloqueada/Campo Email'), Keys.chord(Keys.ENTER))

WebUI.delay(1)

WebUI.takeScreenshot('/Users/huguito/Desktop/notificaciones/Candidato/contratado.png')

WebUI.delay(3)

WebUI.closeBrowser()

def aleatoreo() {
    rn = ((Math.random() * RN) as int)
}

